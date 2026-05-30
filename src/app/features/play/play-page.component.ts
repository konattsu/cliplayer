import { CommonModule } from "@angular/common";
import {
  Component,
  ViewChild,
  computed,
  effect,
  inject,
  signal,
} from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { DomSanitizer } from "@angular/platform-browser";
import { ActivatedRoute, Router, RouterLink } from "@angular/router";

import { LocaleService } from "../../core/i18n/locale.service";
import { DatasetService } from "../../shared/data/dataset.service";
import { formatDuration } from "../../shared/utils/duration";

import { QueueClipViewSchema } from "./queue.schema";
import { QueueService } from "./queue.service";

import type { QueueClipView, QueueState } from "./queue.schema";
import type { ClipUuid } from "../../shared/contracts/common.schema";
import type {
  AppDataset,
  ClipCardView,
} from "../../shared/contracts/dataset.schema";
import type { ElementRef, OnDestroy } from "@angular/core";
import type { SafeResourceUrl } from "@angular/platform-browser";

@Component({
  selector: "app-play-page",
  standalone: true,
  imports: [CommonModule, RouterLink],
  template: `
    @switch (loadState().kind) {
      @case ("booting") {
        <section class="panel loading-panel">
          <h1 i18n="Play boot title@@playBootingTitle">起動中</h1>
        </section>
      }
      @case ("loading_assets") {
        <section class="panel loading-panel">
          <p class="eyebrow" i18n="Play loading eyebrow@@playLoadingEyebrow">
            loading
          </p>
          <h1 i18n="Play loading title@@playLoadingTitle">
            データセットを読み込んでいます
          </h1>
        </section>
      }
      @case ("fatal_error") {
        <section class="panel empty-state">
          <p class="eyebrow" i18n="Play error eyebrow@@playErrorEyebrow">
            load error
          </p>
          <h1 i18n="Play error title@@playErrorTitle">
            データセットを読み込めませんでした
          </h1>
          <p i18n="Play error description@@playErrorDescription">
            時間をおいて再読み込みしてください。技術的な詳細は console に出力しています。
          </p>
        </section>
      }
      @case ("ready") {
        @if (dataset(); as dataset) {
          @if (queue(); as queue) {
            @if (currentClip(); as clip) {
              <section class="watch-page">
                <article class="watch-main">
                  <div class="player-frame">
                    @if (playerSrc(); as src) {
                      <iframe
                        #playerIframe
                        title="YouTube player"
                        [src]="src"
                        allow="autoplay; encrypted-media; picture-in-picture; web-share"
                        allowfullscreen
                      ></iframe>
                    }
                  </div>

                  <div class="progress">
                    <span
                      class="progress-fill"
                      [style.inline-size.%]="progressPercent()"
                    ></span>
                  </div>
                  <div class="playback-time">
                    <span>{{ playbackLabel() }}</span>
                    <span>{{ formatDuration(clip.durationSecs) }}</span>
                  </div>

                  <div class="watch-meta">
                    <h1>{{ clip.songTitle }}</h1>
                    <p>{{ artistLine(clip) }}</p>
                    <p class="muted">
                      {{ clip.video.channelLabel }} ·
                      {{ clip.video.publishedAtIso | date: "yyyy-MM-dd" }} ·
                      {{ formatDuration(clip.durationSecs) }}
                    </p>
                    <p class="muted">{{ clip.video.title }}</p>
                  </div>

                  <div class="player-controls">
                    <button
                      class="button ghost"
                      type="button"
                      (click)="togglePause()"
                    >
                      @if (paused()) {
                        <span i18n="Play button label@@playPlay">再生</span>
                      } @else {
                        <span i18n="Pause button label@@playPause">一時停止</span>
                      }
                    </button>
                    <button
                      class="button ghost"
                      type="button"
                      [disabled]="!canMove(queue, -1)"
                      (click)="move(-1)"
                      i18n="Previous queue item@@playPrevious"
                    >
                      前へ
                    </button>
                    <button
                      class="button primary"
                      type="button"
                      [disabled]="!canMove(queue, 1)"
                      (click)="move(1)"
                      i18n="Next queue item@@playNext"
                    >
                      次へ
                    </button>
                    <a
                      class="button ghost"
                      [href]="youtubeUrl(clip)"
                      target="_blank"
                      rel="noreferrer"
                      i18n="Open original video label@@playOriginal"
                    >
                      元動画
                    </a>
                    <button
                      class="button ghost"
                      type="button"
                      (click)="copyQueueLink(queue)"
                      i18n="Copy queue link label@@playCopyLink"
                    >
                      リンクをコピー
                    </button>
                    @if (shareStatus(); as status) {
                      <span class="status-text">{{ status }}</span>
                    }
                  </div>
                </article>

                <aside class="watch-queue">
                  <div class="queue-heading">
                    <h2 i18n="Queue heading@@playQueueHeading">queue</h2>
                    <span>{{ queue.items.length }}</span>
                  </div>
                  <div class="queue-list">
                    @for (item of queueClipViews(); track item.index) {
                      <article
                        class="queue-item"
                        [class.current]="item.index === queue.currentIndex"
                      >
                        <button
                          class="queue-main"
                          type="button"
                          (click)="jumpTo(item.index)"
                        >
                          <strong>{{ item.clip.songTitle }}</strong>
                          <span>{{ artistLine(item.clip) }}</span>
                        </button>
                        <button
                          class="icon-button"
                          type="button"
                          aria-label="remove item"
                          i18n-aria-label="
                            Remove queue item aria label@@playRemoveItemLabel"
                          (click)="remove(item.index)"
                        >
                          ×
                        </button>
                      </article>
                    }
                  </div>
                </aside>
              </section>
            } @else {
              <section class="empty-state">
                <h1 i18n="Empty queue title@@playEmptyTitle">queue が空です</h1>
                <p
                  class="muted"
                  i18n="Empty queue description@@playEmptyDescription"
                >
                  検索結果または clip URL から queue を作成してください。
                </p>
                <a
                  class="button primary"
                  [routerLink]="['../search']"
                  i18n="Empty queue action@@playEmptyAction"
                >
                  検索を開く
                </a>
              </section>
            }
          }
        }
      }
    }
  `,
})
export class PlayPageComponent implements OnDestroy {
  @ViewChild("playerIframe")
  private playerIframe?: ElementRef<HTMLIFrameElement>;

  private readonly datasetService = inject(DatasetService);
  private readonly queueService = inject(QueueService);
  private readonly route = inject(ActivatedRoute);
  private readonly router = inject(Router);
  private readonly sanitizer = inject(DomSanitizer);
  private readonly localeService = inject(LocaleService);
  private readonly queryParamMap = toSignal(this.route.queryParamMap, {
    initialValue: this.route.snapshot.queryParamMap,
  });
  private initializedKey = "";
  private progressTimer: ReturnType<typeof setInterval> | null = null;
  private progressClipUuid: ClipUuid | null = null;

  readonly paused = signal(false);
  readonly playbackPositionSecs = signal(0);
  readonly shareStatus = signal("");
  readonly loadState = this.datasetService.state;

  readonly dataset = computed(() => {
    const state = this.datasetService.state();
    return state.kind === "ready" ? state.dataset : null;
  });

  readonly locale = this.localeService.locale;
  readonly queue = this.queueService.queue;

  readonly currentClip = computed<ClipCardView | null>(() => {
    const dataset = this.dataset();
    const queue = this.queue();
    if (dataset === null || queue === null || queue.currentIndex === null) {
      return null;
    }

    const item = queue.items[queue.currentIndex];
    return item === undefined
      ? null
      : this.datasetService.joinClip(dataset, item.clipUuid, this.locale());
  });

  readonly queueClipViews = computed<QueueClipView[]>(() => {
    const dataset = this.dataset();
    const queue = this.queue();
    if (dataset === null || queue === null) {
      return [];
    }

    return queue.items
      .map((item, index) => {
        const clip = this.datasetService.joinClip(
          dataset,
          item.clipUuid,
          this.locale(),
        );
        return clip === null
          ? null
          : QueueClipViewSchema.parse({
              index,
              clip,
            });
      })
      .filter((item): item is QueueClipView => item !== null);
  });

  readonly playerSrc = computed<SafeResourceUrl | null>(() => {
    const clip = this.currentClip();
    if (clip === null) {
      return null;
    }

    const params = new URLSearchParams({
      start: String(clip.startTimeSecs),
      end: String(clip.endTimeSecs),
      autoplay: this.paused() ? "0" : "1",
      enablejsapi: "1",
      rel: "0",
    });
    return this.sanitizer.bypassSecurityTrustResourceUrl(
      `https://www.youtube.com/embed/${clip.video.videoId}?${params.toString()}`,
    );
  });

  readonly progressPercent = computed(() => {
    const clip = this.currentClip();
    if (clip === null || clip.durationSecs <= 0) {
      return 0;
    }

    return Math.min(
      100,
      Math.max(0, (this.playbackPositionSecs() / clip.durationSecs) * 100),
    );
  });

  readonly playbackLabel = computed(() =>
    formatDurationValue(this.playbackPositionSecs()),
  );

  constructor() {
    void this.datasetService.load();

    effect(() => {
      const dataset = this.dataset();
      if (dataset === null) {
        return;
      }

      const clipUuid =
        this.route.snapshot.paramMap.get("clipUuid") ??
        this.queryParamMap().get("clip");
      const queueParam = this.queryParamMap().get("q");
      const indexParam = this.queryParamMap().get("i") ?? "0";
      const key = `${dataset.meta.datasetBuildId}:${clipUuid ?? ""}:${queueParam ?? ""}:${indexParam}`;
      if (key === this.initializedKey) {
        return;
      }

      this.initializedKey = key;
      void this.initialize(dataset, clipUuid, queueParam, indexParam);
    });

    effect(() => {
      const clip = this.currentClip();
      const clipUuid = clip?.clipUuid ?? null;
      if (clipUuid === this.progressClipUuid) {
        return;
      }

      this.progressClipUuid = clipUuid;
      this.playbackPositionSecs.set(0);
      this.paused.set(false);
      this.shareStatus.set("");
      this.startProgressTimer();
    });
  }

  ngOnDestroy(): void {
    this.stopProgressTimer();
  }

  canMove(queue: QueueState, delta: -1 | 1): boolean {
    if (queue.currentIndex === null) {
      return false;
    }

    const nextIndex = queue.currentIndex + delta;
    return nextIndex >= 0 && nextIndex < queue.items.length;
  }

  move(delta: -1 | 1): void {
    void this.queueService
      .move(delta)
      .then((queue) => this.syncQueueUrl(queue));
  }

  jumpTo(index: number): void {
    void this.queueService
      .setCurrentIndex(index)
      .then((queue) => this.syncQueueUrl(queue));
  }

  remove(index: number): void {
    void this.queueService
      .removeAt(index)
      .then((queue) => this.syncQueueUrl(queue));
  }

  togglePause(): void {
    const nextPaused = !this.paused();
    this.paused.set(nextPaused);
    this.sendPlayerCommand(nextPaused ? "pauseVideo" : "playVideo");
  }

  async copyQueueLink(queue: QueueState): Promise<void> {
    const url = new URL(window.location.href);
    url.pathname = url.pathname.replace(
      /\/(clip\/[^/]+|play)(?=\/?$)/,
      "/play",
    );
    url.searchParams.set(
      "q",
      queue.items.map((item) => item.clipUuid).join(","),
    );
    url.searchParams.set("i", String(queue.currentIndex ?? 0));

    try {
      await navigator.clipboard.writeText(url.toString());
      this.shareStatus.set(
        $localize`:Queue link copied status@@playCopied:コピーしました`,
      );
    } catch {
      this.shareStatus.set(url.toString());
    }
  }

  artistLine(clip: ClipCardView): string {
    const names = clip.artists.map((artist) =>
      this.locale() === "en" ? artist.nameEn : artist.nameJa,
    );
    return [...names, ...clip.externalArtistsName].join(" / ");
  }

  formatDuration(totalSecs: number): string {
    return formatDuration(totalSecs);
  }

  youtubeUrl(clip: ClipCardView): string {
    return `https://www.youtube.com/watch?v=${clip.video.videoId}&t=${clip.startTimeSecs}s`;
  }

  private async initialize(
    dataset: AppDataset,
    clipUuid: string | null,
    queueParam: string | null,
    indexParam: string,
  ): Promise<void> {
    const queueClipUuids =
      queueParam === null ? [] : parseClipUuids(queueParam, dataset);
    if (queueClipUuids.length > 0) {
      const index = Number(indexParam);
      await this.queueService.createFromClips(
        dataset,
        queueClipUuids,
        { kind: "manual" },
        Number.isInteger(index) ? index : 0,
      );
      return;
    }

    if (clipUuid !== null && dataset.clips[clipUuid] !== undefined) {
      await this.queueService.createFromClip(dataset, clipUuid);
      return;
    }

    await this.queueService.restore(dataset);
  }

  private startProgressTimer(): void {
    this.stopProgressTimer();
    this.progressTimer = setInterval(() => {
      const clip = this.currentClip();
      if (clip === null || this.paused()) {
        return;
      }

      this.playbackPositionSecs.update((current) => {
        const next = Math.min(current + 1, clip.durationSecs);
        if (next >= clip.durationSecs) {
          this.paused.set(true);
          const queue = this.queue();
          if (queue !== null && this.canMove(queue, 1)) {
            setTimeout(() => void this.move(1), 0);
          }
        }

        return next;
      });
    }, 1000);
  }

  private stopProgressTimer(): void {
    if (this.progressTimer !== null) {
      clearInterval(this.progressTimer);
      this.progressTimer = null;
    }
  }

  private sendPlayerCommand(func: "pauseVideo" | "playVideo"): void {
    this.playerIframe?.nativeElement.contentWindow?.postMessage(
      JSON.stringify({
        event: "command",
        func,
        args: [],
      }),
      "https://www.youtube.com",
    );
  }

  private syncQueueUrl(queue: QueueState | null): void {
    if (queue === null || queue.items.length === 0) {
      return;
    }

    void this.router.navigate([], {
      relativeTo: this.route,
      queryParams: {
        q: queue.items.map((item) => item.clipUuid).join(","),
        i: String(queue.currentIndex ?? 0),
      },
      queryParamsHandling: "merge",
      replaceUrl: true,
    });
  }
}

function formatDurationValue(totalSecs: number): string {
  return formatDuration(totalSecs);
}

function parseClipUuids(value: string, dataset: AppDataset): ClipUuid[] {
  return value
    .split(",")
    .map((item) => item.trim())
    .filter((item): item is ClipUuid => dataset.clips[item] !== undefined);
}
