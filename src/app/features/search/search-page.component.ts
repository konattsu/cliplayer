import { CommonModule } from "@angular/common";
import { Component, computed, inject, signal } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { ActivatedRoute, Router, RouterLink } from "@angular/router";

import { LocaleService } from "../../core/i18n/locale.service";
import { DatasetService } from "../../shared/data/dataset.service";
import { formatDuration } from "../../shared/utils/duration";
import { QueueService } from "../play/queue.service";

import {
  ChannelOptionSchema,
  SearchJoinedResultSchema,
  SearchUiStateSchema,
} from "./search.schema";

import type {
  ChannelOption,
  SearchJoinedResult,
  SearchUiState,
} from "./search.schema";
import type {
  AppDataset,
  ClipCardView,
  TagEntity,
} from "../../shared/contracts/dataset.schema";
import type { ParamMap } from "@angular/router";

const DEFAULT_SEARCH_UI = SearchUiStateSchema.parse({
  artistIds: [],
  artistInput: "",
  tagIds: [],
  channelIds: [],
  embeddable: "any",
  isUnlisted: "any",
  publishedFrom: null,
  publishedTo: null,
  sort: "published_at_desc",
  pageCursor: null,
});

@Component({
  selector: "app-search-page",
  standalone: true,
  imports: [CommonModule, FormsModule, RouterLink],
  template: `
    @switch (loadState().kind) {
      @case ("booting") {
        <section class="panel loading-panel">
          <h1 i18n="Search boot title@@searchBootingTitle">起動中</h1>
        </section>
      }
      @case ("loading_assets") {
        <section class="panel loading-panel">
          <p
            class="eyebrow"
            i18n="Search loading eyebrow@@searchLoadingEyebrow"
          >
            loading
          </p>
          <h1 i18n="Search loading title@@searchLoadingTitle">
            データセットを読み込んでいます
          </h1>
        </section>
      }
      @case ("fatal_error") {
        <section class="panel empty-state">
          <p class="eyebrow" i18n="Search error eyebrow@@searchErrorEyebrow">
            load error
          </p>
          <h1 i18n="Search error title@@searchErrorTitle">
            データセットを読み込めませんでした
          </h1>
          <p i18n="Search error description@@searchErrorDescription">
            時間をおいて再読み込みしてください。技術的な詳細は console
            に出力しています。
          </p>
        </section>
      }
      @case ("ready") {
        @if (dataset(); as dataset) {
          <section class="search-page">
            <form class="filter-panel" (submit)="$event.preventDefault()">
              <div class="page-heading">
                <p class="eyebrow" i18n="Search page eyebrow@@searchEyebrow">
                  検索
                </p>
                <h1 i18n="Search page title@@searchTitle">clip を探す</h1>
                <p class="muted" i18n="Search page intro@@searchIntro">
                  件数を確認してから、確定して clip を表示します。
                </p>
              </div>

              <label class="field">
                <span i18n="Search field label@@searchArtistLabel">artist</span>
                <input
                  class="input"
                  type="search"
                  role="combobox"
                  aria-autocomplete="list"
                  aria-controls="artist-suggestions"
                  [attr.aria-expanded]="artistSuggestions().length > 0"
                  [ngModel]="artistInput()"
                  (ngModelChange)="setArtistInput($event)"
                  (keydown)="handleArtistKeydown($event)"
                  placeholder="ライバー名"
                  i18n-placeholder="
                    Search artist placeholder@@searchArtistPlaceholder"
                  name="artist"
                />
              </label>

              @if (artistSuggestions().length > 0) {
                <div class="suggestions" id="artist-suggestions" role="listbox">
                  @for (
                    suggestion of artistSuggestions();
                    track suggestion.artistId
                  ) {
                    <button
                      class="suggestion"
                      type="button"
                      role="option"
                      (click)="addArtist(suggestion.artistId)"
                    >
                      <span>{{ suggestion.label }}</span>
                      <small>{{ suggestion.matchedKey }}</small>
                    </button>
                  }
                </div>
              }

              @if (draft().artistIds.length > 0) {
                <div class="chip-list">
                  @for (artistId of draft().artistIds; track artistId) {
                    <button
                      class="chip removable"
                      type="button"
                      (click)="removeArtist(artistId)"
                    >
                      {{ artistName(dataset, artistId) }} ×
                    </button>
                  }
                </div>
              }

              <details class="selector">
                <summary>
                  <span i18n="Search tag label@@searchTagLabel">tag</span>
                  <strong>{{ draft().tagIds.length }}</strong>
                </summary>
                <div class="selector-body">
                  @for (tag of tagOptions(); track tag.tagId) {
                    <button
                      class="selector-option"
                      type="button"
                      [class.selected]="draft().tagIds.includes(tag.tagId)"
                      (click)="toggleTag(tag.tagId)"
                    >
                      {{ tagName(tag) }}
                    </button>
                  }
                </div>
              </details>

              <label class="field">
                <span i18n="Search channel label@@searchChannelLabel">
                  channel
                </span>
                <input
                  class="input"
                  type="search"
                  role="combobox"
                  aria-autocomplete="list"
                  aria-controls="channel-suggestions"
                  [attr.aria-expanded]="channelSuggestions().length > 0"
                  [ngModel]="channelInput()"
                  (ngModelChange)="setChannelInput($event)"
                  placeholder="チャンネル名"
                  i18n-placeholder="
                    Search channel placeholder@@searchChannelPlaceholder"
                  name="channel"
                />
              </label>

              @if (channelSuggestions().length > 0) {
                <div
                  class="suggestions"
                  id="channel-suggestions"
                  role="listbox"
                >
                  @for (
                    channel of channelSuggestions();
                    track channel.channelId
                  ) {
                    <button
                      class="suggestion"
                      type="button"
                      role="option"
                      (click)="addChannel(channel.channelId)"
                    >
                      <span>{{ channel.label }}</span>
                      <small>{{ channel.matchedKey }}</small>
                    </button>
                  }
                </div>
              }

              @if (draft().channelIds.length > 0) {
                <div class="chip-list">
                  @for (channelId of draft().channelIds; track channelId) {
                    <button
                      class="chip removable"
                      type="button"
                      (click)="removeChannel(channelId)"
                    >
                      {{ channelName(channelId) }} ×
                    </button>
                  }
                </div>
              }

              <div class="field-grid">
                <label class="field">
                  <span i18n="Search from month label@@searchFromMonth">
                    from month
                  </span>
                  <input
                    class="input"
                    type="month"
                    [ngModel]="monthValue(draft().publishedFrom)"
                    name="from"
                    (ngModelChange)="setFromMonth($event)"
                  />
                </label>
                <label class="field">
                  <span i18n="Search to month label@@searchToMonth">
                    to month
                  </span>
                  <input
                    class="input"
                    type="month"
                    [ngModel]="monthValue(draft().publishedTo)"
                    name="to"
                    (ngModelChange)="setToMonth($event)"
                  />
                </label>
              </div>

              <div class="field-grid">
                <label class="field">
                  <span i18n="Search embeddable label@@searchEmbedLabel">
                    embed
                  </span>
                  <select
                    class="input"
                    [ngModel]="draft().embeddable"
                    name="embed"
                    (ngModelChange)="setEmbeddable($event)"
                  >
                    <option value="any" i18n="Search tri-state any@@searchAny">
                      any
                    </option>
                    <option
                      value="only"
                      i18n="Search tri-state only@@searchOnly"
                    >
                      only
                    </option>
                    <option
                      value="exclude"
                      i18n="Search tri-state exclude@@searchExclude"
                    >
                      exclude
                    </option>
                  </select>
                </label>
                <label class="field">
                  <span i18n="Search sort label@@searchSortLabel">sort</span>
                  <select
                    class="input"
                    [ngModel]="draft().sort"
                    name="sort"
                    (ngModelChange)="setSort($event)"
                  >
                    <option
                      value="published_at_desc"
                      i18n="Search newest sort label@@searchNewest"
                    >
                      新しい順
                    </option>
                    <option
                      value="published_at_asc"
                      i18n="Search oldest sort label@@searchOldest"
                    >
                      古い順
                    </option>
                  </select>
                </label>
              </div>

              <div class="search-actions">
                <div class="result-count">
                  <strong>{{ previewResult().total }}</strong>
                  <span i18n="Clips count unit@@searchClipsUnit">clips</span>
                </div>
                <button
                  class="button primary"
                  type="button"
                  (click)="confirm()"
                  i18n="Show clips button@@searchShowClips"
                >
                  clip を表示
                </button>
                <button
                  class="button ghost"
                  type="button"
                  (click)="clearFilters()"
                  i18n="Clear search button@@searchClear"
                >
                  クリア
                </button>
              </div>
            </form>

            <section class="result-panel">
              @if (!confirmed()) {
                <div class="empty-state">
                  <h2>{{ previewResult().total }} clips</h2>
                  <p
                    class="muted"
                    i18n="Search confirm hint@@searchConfirmHint"
                  >
                    条件を確定すると具体的な clip 一覧を表示します。
                  </p>
                </div>
              } @else {
                <div class="section-heading">
                  <div>
                    <p
                      class="eyebrow"
                      i18n="Search results eyebrow@@searchResultsEyebrow"
                    >
                      results
                    </p>
                    <h2>{{ visibleResult().total }} clips</h2>
                  </div>
                  <button
                    class="button primary"
                    type="button"
                    [disabled]="visibleResult().clipCards.length === 0"
                    (click)="playResult(visibleResult())"
                    i18n="Search queue action@@searchQueueAction"
                  >
                    この結果で再生
                  </button>
                </div>

                <div class="clip-list">
                  @for (
                    clip of visibleResult().clipCards;
                    track clip.clipUuid
                  ) {
                    <article class="clip-row">
                      <div class="clip-card-main">
                        <h3>{{ clip.songTitle }}</h3>
                        <p>{{ artistLine(clip) }}</p>
                        <p class="muted">{{ clip.video.title }}</p>
                        <p class="muted">
                          {{ clip.video.channelLabel }} ·
                          {{ clip.video.publishedAtIso | date: "yyyy-MM-dd" }} ·
                          {{ formatDuration(clip.durationSecs) }}
                        </p>
                      </div>
                      <div class="card-actions">
                        <a
                          class="button compact"
                          [routerLink]="['../clip', clip.clipUuid]"
                          i18n="Open clip action@@searchOpenClip"
                          >開く</a
                        >
                        <button
                          class="button compact ghost"
                          type="button"
                          (click)="addToQueue(clip.clipUuid)"
                          i18n="Add to queue action@@searchAddToQueue"
                        >
                          キューに追加
                        </button>
                      </div>
                    </article>
                  } @empty {
                    <div class="empty-state">
                      <h3 i18n="Empty search results title@@searchNoClips">
                        該当なし
                      </h3>
                    </div>
                  }
                </div>
              }
            </section>
          </section>
        }
      }
    }
  `,
})
export class SearchPageComponent {
  private readonly datasetService = inject(DatasetService);
  private readonly queueService = inject(QueueService);
  private readonly route = inject(ActivatedRoute);
  private readonly router = inject(Router);
  private readonly localeService = inject(LocaleService);

  readonly draft = signal<SearchUiState>({
    ...DEFAULT_SEARCH_UI,
    ...parseSearchParams(this.route.snapshot.queryParamMap),
  });
  readonly artistInput = signal("");
  readonly channelInput = signal("");
  readonly confirmed = signal(
    this.route.snapshot.queryParamMap.get("apply") === "1",
  );
  readonly loadState = this.datasetService.state;

  readonly dataset = computed(() => {
    const state = this.datasetService.state();
    return state.kind === "ready" ? state.dataset : null;
  });

  readonly locale = this.localeService.locale;
  readonly previewResult = computed<SearchJoinedResult>(() => {
    const dataset = this.dataset();
    return dataset === null
      ? emptyResult()
      : this.datasetService.search(dataset, this.draft());
  });

  readonly visibleResult = computed<SearchJoinedResult>(() =>
    this.confirmed() ? this.previewResult() : emptyResult(),
  );

  readonly artistSuggestions = computed(() => {
    const dataset = this.dataset();
    return dataset === null
      ? []
      : this.datasetService.suggestArtists(dataset, this.artistInput());
  });

  readonly tagOptions = computed<TagEntity[]>(() => {
    const dataset = this.dataset();
    return dataset === null
      ? []
      : Object.values(dataset.tags).filter((tag) => !tag.blocked);
  });

  readonly channelSuggestions = computed<ChannelOption[]>(() => {
    const dataset = this.dataset();
    return dataset === null
      ? []
      : this.datasetService
          .suggestChannels(dataset, this.channelInput(), this.locale())
          .map((option) => ChannelOptionSchema.parse(option));
  });

  constructor() {
    void this.datasetService.load();
  }

  setArtistInput(value: string): void {
    this.artistInput.set(value);
  }

  addArtist(artistId: string): void {
    this.artistInput.set("");
    this.patchDraft({ artistIds: [...this.draft().artistIds, artistId] });
  }

  removeArtist(artistId: string): void {
    this.patchDraft({
      artistIds: this.draft().artistIds.filter(
        (current) => current !== artistId,
      ),
    });
  }

  toggleTag(tagId: string): void {
    const tagIds = this.draft().tagIds.includes(tagId)
      ? this.draft().tagIds.filter((current) => current !== tagId)
      : [...this.draft().tagIds, tagId];
    this.patchDraft({ tagIds });
  }

  setChannelInput(value: string): void {
    this.channelInput.set(value);
  }

  addChannel(channelId: string): void {
    this.channelInput.set("");
    this.patchDraft({ channelIds: [...this.draft().channelIds, channelId] });
  }

  removeChannel(channelId: string): void {
    this.patchDraft({
      channelIds: this.draft().channelIds.filter(
        (current) => current !== channelId,
      ),
    });
  }

  setFromMonth(value: string): void {
    this.patchDraft({
      publishedFrom: value.length === 0 ? null : `${value}-01`,
    });
  }

  setToMonth(value: string): void {
    this.patchDraft({
      publishedTo: value.length === 0 ? null : lastDayOfMonth(value),
    });
  }

  setEmbeddable(value: string): void {
    this.patchDraft({ embeddable: normalizeTriState(value) });
  }

  setSort(value: string): void {
    this.patchDraft({
      sort:
        value === "published_at_asc" || value === "published_at_desc"
          ? value
          : "published_at_desc",
    });
  }

  clearFilters(): void {
    this.artistInput.set("");
    this.channelInput.set("");
    this.draft.set(DEFAULT_SEARCH_UI);
    this.confirmed.set(false);
    void this.router.navigate([], { relativeTo: this.route, queryParams: {} });
  }

  confirm(): void {
    this.confirmed.set(true);
    void this.router.navigate([], {
      relativeTo: this.route,
      queryParams: serializeSearchParams(this.draft()),
    });
  }

  handleArtistKeydown(event: KeyboardEvent): void {
    if (event.key !== "Enter") {
      return;
    }

    const suggestion = this.artistSuggestions()[0];
    if (suggestion === undefined) {
      return;
    }

    event.preventDefault();
    this.addArtist(suggestion.artistId);
  }

  playResult(result: SearchJoinedResult): void {
    const clipUuids = result.clipCards.map((clip) => clip.clipUuid);
    void this.router.navigate(["../play"], {
      relativeTo: this.route,
      queryParams: {
        q: clipUuids.join(","),
        i: "0",
      },
    });
  }

  addToQueue(clipUuid: string): void {
    const dataset = this.dataset();
    if (dataset === null) {
      return;
    }

    void this.queueService.addClip(dataset, clipUuid);
  }

  artistName(dataset: AppDataset, artistId: string): string {
    const artist = dataset.artists[artistId];
    if (artist === undefined) {
      return artistId;
    }

    return this.locale() === "en" ? artist.nameEn : artist.nameJa;
  }

  artistLine(clip: ClipCardView): string {
    const names = clip.artists.map((artist) =>
      this.locale() === "en" ? artist.nameEn : artist.nameJa,
    );
    return [...names, ...clip.externalArtistsName].join(" / ");
  }

  tagName(tag: TagEntity): string {
    return this.locale() === "en" ? tag.nameEn : tag.nameJa;
  }

  channelName(channelId: string): string {
    const dataset = this.dataset();
    return dataset === null
      ? channelId
      : this.datasetService.channelLabelById(dataset, channelId, this.locale());
  }

  monthValue(value: string | null): string {
    return value === null ? "" : value.slice(0, 7);
  }

  formatDuration(totalSecs: number): string {
    return formatDuration(totalSecs);
  }

  private patchDraft(patch: Partial<SearchUiState>): void {
    this.confirmed.set(false);
    this.draft.update((current) => ({
      ...current,
      ...patch,
      pageCursor: null,
    }));
  }
}

function emptyResult(): SearchJoinedResult {
  return SearchJoinedResultSchema.parse({
    clipCards: [],
    nextCursor: null,
    totalMode: "exact",
    total: 0,
    hasMore: false,
    warnings: [],
  });
}

function parseSearchParams(paramMap: ParamMap): Partial<SearchUiState> {
  return {
    artistIds: parseCsv(paramMap.get("artist")),
    tagIds: parseCsv(paramMap.get("tag")),
    channelIds: parseCsv(paramMap.get("channel")),
    embeddable: normalizeTriState(paramMap.get("embed") ?? "any"),
    isUnlisted: normalizeTriState(paramMap.get("unlisted") ?? "any"),
    publishedFrom: paramMap.get("from"),
    publishedTo: paramMap.get("to"),
    sort:
      paramMap.get("sort") === "published_at_asc"
        ? "published_at_asc"
        : "published_at_desc",
    pageCursor: paramMap.get("pageCursor"),
  };
}

function serializeSearchParams(
  ui: SearchUiState,
): Record<string, string | null> {
  return {
    artist: csv(ui.artistIds),
    tag: csv(ui.tagIds),
    channel: csv(ui.channelIds),
    embed: ui.embeddable === "any" ? null : ui.embeddable,
    unlisted: ui.isUnlisted === "any" ? null : ui.isUnlisted,
    from: ui.publishedFrom,
    to: ui.publishedTo,
    sort: ui.sort === "published_at_desc" ? null : ui.sort,
    pageCursor: null,
    apply: "1",
  };
}

function parseCsv(value: string | null): string[] {
  if (value === null || value.length === 0) {
    return [];
  }

  return Array.from(
    new Set(
      value
        .split(",")
        .map((item) => item.trim())
        .filter((item) => item.length > 0),
    ),
  ).sort();
}

function csv(values: string[]): string | null {
  const normalized = Array.from(new Set(values)).sort();
  return normalized.length === 0 ? null : normalized.join(",");
}

function normalizeTriState(value: string): "any" | "only" | "exclude" {
  return value === "only" || value === "exclude" ? value : "any";
}

function lastDayOfMonth(value: string): string {
  const [yearString, monthString] = value.split("-");
  const year = Number(yearString);
  const month = Number(monthString);
  if (!Number.isInteger(year) || !Number.isInteger(month)) {
    return `${value}-01`;
  }

  const lastDate = new Date(Date.UTC(year, month, 0)).getUTCDate();
  return `${value}-${lastDate.toString().padStart(2, "0")}`;
}
