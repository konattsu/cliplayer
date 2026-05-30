import { CommonModule } from "@angular/common";
import { Component, inject } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { FormsModule } from "@angular/forms";
import { Router, RouterLink } from "@angular/router";

import { LocaleService } from "../../core/i18n/locale.service";
import { switchLocaleUrl } from "../../core/locale/locale.helpers";

import { SettingsService } from "./settings.service";

import type { ThemePreference } from "./settings.schema";

@Component({
  selector: "app-settings-page",
  standalone: true,
  imports: [CommonModule, FormsModule, RouterLink],
  template: `
    <section class="settings-page">
      <div class="page-heading">
        <p class="eyebrow" i18n="Settings page eyebrow@@settingsEyebrow">
          settings
        </p>
        <h1 i18n="Settings page title@@settingsTitle">設定</h1>
        <p class="muted" i18n="Settings page description@@settingsDescription">
          言語は URL で切り替え、テーマと ambient color をここで調整します。
        </p>
      </div>

      <form class="settings-form" (submit)="$event.preventDefault()">
        <label class="field">
          <span i18n="Settings language label@@settingsLanguage">言語</span>
          <select
            class="input"
            [ngModel]="locale()"
            name="locale"
            (ngModelChange)="setLocale($event)"
          >
            <option value="ja">日本語</option>
            <option value="en">English</option>
          </select>
        </label>

        <label class="field">
          <span i18n="Settings theme label@@settingsTheme">テーマ</span>
          <select
            class="input"
            [ngModel]="preferences().theme"
            name="theme"
            (ngModelChange)="setTheme($event)"
          >
            <option
              value="system"
              i18n="Settings system theme option@@settingsThemeSystem"
            >
              system
            </option>
            <option
              value="light"
              i18n="Settings light theme option@@settingsThemeLight"
            >
              light
            </option>
            <option
              value="dark"
              i18n="Settings dark theme option@@settingsThemeDark"
            >
              dark
            </option>
          </select>
        </label>

        <label class="field">
          <span i18n="Settings ambient color label@@settingsAccent">
            ambient color
          </span>
          <input
            class="input color-input"
            type="color"
            [ngModel]="preferences().ambientColorHex ?? '#6f8ea2'"
            name="ambientColorHex"
            (ngModelChange)="setAmbientColor($event)"
          />
          <small
            class="muted"
            i18n="Settings ambient color help@@settingsAccentHelp"
          >
            背景の淡い色味や装飾にだけ使います。本文や主要ボタンには使いません。
          </small>
        </label>

        <div class="cluster">
          <button class="button primary" type="button" (click)="goBack()">
            {{ backLabel() }}
          </button>
          <button
            class="button ghost"
            type="button"
            (click)="resetAmbientColor()"
            i18n="Settings reset accent label@@settingsResetAccent"
          >
            既定色に戻す
          </button>
          <a
            class="button ghost"
            [routerLink]="['/search']"
            i18n="Settings open search label@@settingsOpenSearch"
          >
            検索を開く
          </a>
        </div>
      </form>
    </section>
  `,
})
export class SettingsPageComponent {
  private readonly settings = inject(SettingsService);
  private readonly router = inject(Router);
  private readonly localeService = inject(LocaleService);

  readonly preferences = toSignal(this.settings.preferences$, {
    initialValue: this.settings.snapshot(),
  });

  readonly locale = this.localeService.locale;

  setLocale(value: string): void {
    const locale = value === "en" ? "en" : "ja";
    window.location.assign(
      switchLocaleUrl(
        locale,
        window.location.pathname,
        window.location.search,
        window.location.hash,
      ),
    );
  }

  setTheme(value: string): void {
    const theme: ThemePreference =
      value === "light" || value === "dark" || value === "system"
        ? value
        : "system";
    this.settings.update({ theme });
  }

  setAmbientColor(value: string): void {
    this.settings.update({ ambientColorHex: value });
  }

  resetAmbientColor(): void {
    this.settings.update({ ambientColorHex: null });
  }

  backLabel(): string {
    const target = this.backDestinationLabel();
    return target === null
      ? $localize`:Settings back home label@@settingsBackHome:ホームへ戻る`
      : `${$localize`:Settings back to prefix@@settingsBackTo:戻る:`} ${target}`;
  }

  goBack(): void {
    const referrer = document.referrer;
    if (referrer.length > 0) {
      try {
        const url = new URL(referrer);
        if (url.origin === window.location.origin) {
          window.location.assign(url.pathname + url.search + url.hash);
          return;
        }
      } catch {
        // Fall through to the locale-aware search page.
      }
    }

    void this.router.navigate(["/"]);
  }

  private backDestinationLabel(): string | null {
    const referrer = document.referrer;
    if (referrer.length === 0) {
      return null;
    }

    try {
      const url = new URL(referrer);
      if (url.origin !== window.location.origin) {
        return null;
      }

      if (/\/search(?:\/|$)/.test(url.pathname)) {
        return $localize`:Settings back destination search@@settingsBackDestSearch:検索`;
      }
      if (/\/play(?:\/|$)|\/clip\//.test(url.pathname)) {
        return $localize`:Settings back destination play@@settingsBackDestPlay:再生`;
      }
      if (/\/settings(?:\/|$)/.test(url.pathname)) {
        return $localize`:Settings back destination settings@@settingsBackDestSettings:設定`;
      }
      return $localize`:Settings back destination home@@settingsBackDestHome:ホーム`;
    } catch {
      return null;
    }
  }
}
