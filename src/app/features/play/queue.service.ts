import { Injectable, signal } from "@angular/core";

import { QueueStateSchema, PersistedQueueV1Schema } from "./queue.schema";

import type {
  PersistedQueueV1,
  QueueItem,
  QueueOrigin,
  QueueState,
} from "./queue.schema";
import type { ClipUuid } from "../../shared/contracts/common.schema";
import type { AppDataset } from "../../shared/contracts/dataset.schema";

const DB_NAME = "cliplayer";
const DB_VERSION = 1;
const QUEUE_STORE = "queues";
const CURRENT_QUEUE_KEY = "current";

@Injectable({ providedIn: "root" })
export class QueueService {
  readonly queue = signal<QueueState | null>(null);

  async restore(dataset: AppDataset): Promise<QueueState> {
    const persisted = await readPersistedQueue();
    if (
      persisted === null ||
      persisted.version !== 1 ||
      persisted.datasetBuildId !== dataset.meta.datasetBuildId
    ) {
      const empty = createEmptyQueue(dataset.meta.datasetBuildId);
      this.queue.set(empty);
      return empty;
    }

    const items = persisted.queue.items.filter(
      (item) => dataset.clips[item.clipUuid] !== undefined,
    );
    const currentIndex =
      items.length === 0
        ? null
        : persisted.queue.currentIndex !== null &&
            persisted.queue.currentIndex >= 0 &&
            persisted.queue.currentIndex < items.length
          ? persisted.queue.currentIndex
          : 0;
    const queue = QueueStateSchema.parse({
      ...persisted.queue,
      items,
      currentIndex,
      datasetBuildId: dataset.meta.datasetBuildId,
    });
    this.queue.set(queue);
    return queue;
  }

  async createFromClip(
    dataset: AppDataset,
    clipUuid: ClipUuid,
  ): Promise<QueueState> {
    const queue = createQueue(dataset.meta.datasetBuildId, [{ clipUuid }], {
      kind: "single_clip",
      clipUuid,
    });
    await this.replace(queue);
    return queue;
  }

  async createFromClips(
    dataset: AppDataset,
    clipUuids: ClipUuid[],
    origin: QueueOrigin,
    currentIndex = 0,
  ): Promise<QueueState> {
    const queue = createQueue(
      dataset.meta.datasetBuildId,
      clipUuids.map((clipUuid) => ({ clipUuid })),
      origin,
      currentIndex,
    );
    await this.replace(queue);
    return queue;
  }

  async addClip(dataset: AppDataset, clipUuid: ClipUuid): Promise<QueueState> {
    const current =
      this.queue() ?? createEmptyQueue(dataset.meta.datasetBuildId);
    const now = Date.now();
    const next = QueueStateSchema.parse({
      ...current,
      items: [...current.items, { clipUuid, addedAtEpochMs: now }],
      currentIndex: current.currentIndex ?? 0,
      updatedAtEpochMs: now,
    });
    await this.replace(next);
    return next;
  }

  async move(delta: -1 | 1): Promise<QueueState | null> {
    const current = this.queue();
    if (current === null || current.currentIndex === null) {
      return current;
    }

    const nextIndex = current.currentIndex + delta;
    if (nextIndex < 0 || nextIndex >= current.items.length) {
      return current;
    }

    const next = QueueStateSchema.parse({
      ...current,
      currentIndex: nextIndex,
      updatedAtEpochMs: Date.now(),
    });
    await this.replace(next);
    return next;
  }

  async removeAt(index: number): Promise<QueueState | null> {
    const current = this.queue();
    if (current === null || index < 0 || index >= current.items.length) {
      return current;
    }

    const items = current.items.filter(
      (_item, itemIndex) => itemIndex !== index,
    );
    const currentIndex =
      items.length === 0
        ? null
        : current.currentIndex === null
          ? 0
          : Math.min(current.currentIndex, items.length - 1);
    const next = QueueStateSchema.parse({
      ...current,
      items,
      currentIndex,
      updatedAtEpochMs: Date.now(),
    });
    await this.replace(next);
    return next;
  }

  async setCurrentIndex(index: number): Promise<QueueState | null> {
    const current = this.queue();
    if (current === null || index < 0 || index >= current.items.length) {
      return current;
    }

    const next = QueueStateSchema.parse({
      ...current,
      currentIndex: index,
      updatedAtEpochMs: Date.now(),
    });
    await this.replace(next);
    return next;
  }

  private async replace(queue: QueueState): Promise<void> {
    this.queue.set(queue);
    await writePersistedQueue(
      PersistedQueueV1Schema.parse({
        version: 1,
        savedAtEpochMs: Date.now(),
        datasetBuildId: queue.datasetBuildId,
        queue,
      }),
    );
  }
}

function createQueue(
  datasetBuildId: string,
  items: Array<{ clipUuid: ClipUuid }>,
  origin: QueueOrigin,
  currentIndex = 0,
): QueueState {
  const now = Date.now();
  const queueItems: QueueItem[] = items.map((item) => ({
    clipUuid: item.clipUuid,
    addedAtEpochMs: now,
  }));

  return QueueStateSchema.parse({
    queueId: crypto.randomUUID(),
    datasetBuildId,
    items: queueItems,
    currentIndex:
      queueItems.length === 0
        ? null
        : Math.min(Math.max(currentIndex, 0), queueItems.length - 1),
    origin,
    updatedAtEpochMs: now,
  });
}

function createEmptyQueue(datasetBuildId: string): QueueState {
  return createQueue(datasetBuildId, [], { kind: "manual" });
}

async function readPersistedQueue(): Promise<PersistedQueueV1 | null> {
  const db = await openDb();
  return new Promise((resolve, reject) => {
    const request = db
      .transaction(QUEUE_STORE, "readonly")
      .objectStore(QUEUE_STORE)
      .get(CURRENT_QUEUE_KEY);

    request.onsuccess = (): void => {
      try {
        const parsed =
          request.result === undefined || request.result === null
            ? null
            : PersistedQueueV1Schema.parse(request.result);
        resolve(parsed);
      } catch {
        resolve(null);
      } finally {
        db.close();
      }
    };
    request.onerror = (): void => {
      reject(request.error ?? new Error("Failed to read queue"));
      db.close();
    };
  });
}

async function writePersistedQueue(queue: PersistedQueueV1): Promise<void> {
  const db = await openDb();
  return new Promise((resolve, reject) => {
    const request = db
      .transaction(QUEUE_STORE, "readwrite")
      .objectStore(QUEUE_STORE)
      .put(queue, CURRENT_QUEUE_KEY);

    request.onsuccess = (): void => {
      resolve();
      db.close();
    };
    request.onerror = (): void => {
      reject(request.error ?? new Error("Failed to write queue"));
      db.close();
    };
  });
}

function openDb(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(DB_NAME, DB_VERSION);

    request.onupgradeneeded = (): void => {
      const db = request.result;
      if (!db.objectStoreNames.contains(QUEUE_STORE)) {
        db.createObjectStore(QUEUE_STORE);
      }
    };

    request.onsuccess = (): void => resolve(request.result);
    request.onerror = (): void =>
      reject(request.error ?? new Error("Failed to open DB"));
  });
}
