import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";

import type { UserPreferences } from "./settings.schema";

const STORAGE_KEY = "cliplayer:user-preferences:v1";
const DEFAULT_PREFERENCES: UserPreferences = {
  version: 1,
  locale: "ja",
  theme: "system",
  ambientColorHex: null,
  searchMode: "basic",
  filterGraduatedArtists: "disabled",
};

@Injectable({ providedIn: "root" })
export class SettingsService {
  private readonly subject = new BehaviorSubject<UserPreferences>(
    loadPreferences(),
  );

  readonly preferences$ = this.subject.asObservable();

  snapshot(): UserPreferences {
    return this.subject.value;
  }

  update(patch: Partial<UserPreferences>): void {
    const next = normalizePreferences({
      ...this.subject.value,
      ...patch,
      version: 1,
    });
    this.subject.next(next);
    localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
    applyPreferences(next);
  }

  applyCurrentTheme(): void {
    applyPreferences(this.subject.value);
  }
}

function loadPreferences(): UserPreferences {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (raw === null) {
    return DEFAULT_PREFERENCES;
  }

  try {
    return normalizePreferences(JSON.parse(raw));
  } catch {
    return DEFAULT_PREFERENCES;
  }
}

function normalizePreferences(value: unknown): UserPreferences {
  if (!isRecord(value)) {
    return DEFAULT_PREFERENCES;
  }

  return {
    version: 1,
    locale: value["locale"] === "en" ? "en" : "ja",
    theme:
      value["theme"] === "light" ||
      value["theme"] === "dark" ||
      value["theme"] === "system"
        ? value["theme"]
        : "system",
    ambientColorHex: normalizeAmbientColorHex(value["ambientColorHex"]),
    searchMode: "basic",
    filterGraduatedArtists:
      value["filterGraduatedArtists"] === "strict" ||
      value["filterGraduatedArtists"] === "loose" ||
      value["filterGraduatedArtists"] === "disabled"
        ? value["filterGraduatedArtists"]
        : "disabled",
  };
}

function normalizeAmbientColorHex(value: unknown): string | null {
  if (value === null) {
    return null;
  }

  return typeof value === "string" && /^#[0-9a-fA-F]{6}$/.test(value)
    ? value
    : null;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function applyPreferences(preferences: UserPreferences): void {
  const { theme, ambientColorHex } = preferences;
  const resolved =
    theme === "system"
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light"
      : theme;
  document.documentElement.dataset["theme"] = resolved;
  document.documentElement.style.setProperty(
    "--ambient-color",
    ambientColorHex ?? "#6f8ea2",
  );
}
