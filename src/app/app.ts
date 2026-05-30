import { CommonModule } from "@angular/common";
import { Component, effect, inject, signal } from "@angular/core";
import { RouterLink, RouterLinkActive, RouterOutlet } from "@angular/router";

import { LocaleService } from "./core/i18n/locale.service";
import { switchLocaleUrl } from "./core/locale/locale.helpers";
import { SettingsService } from "./features/settings/settings.service";

@Component({
  selector: "app-root",
  standalone: true,
  imports: [CommonModule, RouterLink, RouterLinkActive, RouterOutlet],
  template: `
    <div class="site-frame" [class.play-mode]="isPlayRoute()">
      <header class="site-header">
        <a class="brand" [routerLink]="['/']">
          <span class="brand-mark">clip</span>
          <span>cliplayer</span>
        </a>
        <button
          class="menu-toggle"
          type="button"
          [attr.aria-expanded]="menuOpen()"
          aria-label="ナビゲーションを開く"
          i18n-aria-label="Mobile navigation aria label@@navMenuLabel"
          (click)="toggleMenu()"
        >
          <span i18n="Mobile navigation button label@@navMenu">メニュー</span>
        </button>
        <nav
          class="site-nav"
          aria-label="primary"
          i18n-aria-label="Primary navigation aria label@@navPrimaryLabel"
          [class.open]="menuOpen()"
        >
          <a
            [routerLink]="['/search']"
            routerLinkActive="active"
            (click)="closeMenu()"
            i18n="Primary navigation label@@navSearch"
            >検索</a
          >
          <a
            [routerLink]="['/play']"
            routerLinkActive="active"
            (click)="closeMenu()"
            i18n="Primary navigation label@@navPlay"
            >再生</a
          >
          <a
            [routerLink]="['/settings']"
            routerLinkActive="active"
            (click)="closeMenu()"
            i18n="Primary navigation label@@navSettings"
            >設定</a
          >
        </nav>
        <div
          class="locale-switch"
          aria-label="locale"
          i18n-aria-label="Locale switch aria label@@navLocaleSwitchLabel"
        >
          <a [href]="switchLocaleHref('ja')">JA</a>
          <a [href]="switchLocaleHref('en')">EN</a>
        </div>
      </header>

      <main class="page-shell">
        <router-outlet />
      </main>

      <footer class="site-footer">
        <span>cliplayer</span>
      </footer>
    </div>
  `,
})
export class App {
  private readonly settingsService = inject(SettingsService);
  private readonly localeService = inject(LocaleService);

  readonly menuOpen = signal(false);

  constructor() {
    this.settingsService.applyCurrentTheme();
    effect(() => {
      document.documentElement.lang = this.activeLocale();
    });
  }

  switchLocaleHref(locale: "ja" | "en"): string {
    return switchLocaleUrl(
      locale,
      window.location.pathname,
      window.location.search,
      window.location.hash,
    );
  }

  activeLocale(): "ja" | "en" {
    return this.localeService.locale();
  }

  toggleMenu(): void {
    this.menuOpen.update((current) => !current);
  }

  closeMenu(): void {
    this.menuOpen.set(false);
  }

  isPlayRoute(): boolean {
    return /^(?:\/(?:ja|en))?\/(play|clip)(?=\/|\?|$)/.test(
      window.location.pathname,
    );
  }
}
