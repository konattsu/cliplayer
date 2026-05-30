import { z } from "zod";

import {
  ClipUuidSchema,
  DatasetBuildIdSchema,
} from "../../shared/contracts/common.schema";
import {
  AppDatasetSchema,
  ClipCardViewSchema,
} from "../../shared/contracts/dataset.schema";

export const QueueItemSchema = z.object({
  clipUuid: ClipUuidSchema,
  addedAtEpochMs: z.number().int(),
});

export const QueueOriginSchema = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("manual") }),
  z.object({
    kind: z.literal("single_clip"),
    clipUuid: ClipUuidSchema,
  }),
  z.object({
    kind: z.literal("search_result"),
    searchFingerprint: z.string(),
  }),
]);

export const QueueStateSchema = z.object({
  queueId: z.string().min(1),
  datasetBuildId: DatasetBuildIdSchema,
  items: z.array(QueueItemSchema),
  currentIndex: z.number().int().nullable(),
  origin: QueueOriginSchema,
  updatedAtEpochMs: z.number().int(),
});

export const PersistedQueueV1Schema = z.object({
  version: z.literal(1),
  savedAtEpochMs: z.number().int(),
  datasetBuildId: DatasetBuildIdSchema,
  queue: QueueStateSchema,
});

export const QueueClipViewSchema = z.object({
  index: z.number().int(),
  clip: ClipCardViewSchema,
});

export const QueueDatasetInputSchema = AppDatasetSchema;

export type QueueItem = z.infer<typeof QueueItemSchema>;
export type QueueOrigin = z.infer<typeof QueueOriginSchema>;
export type QueueState = z.infer<typeof QueueStateSchema>;
export type PersistedQueueV1 = z.infer<typeof PersistedQueueV1Schema>;
export type QueueClipView = z.infer<typeof QueueClipViewSchema>;
