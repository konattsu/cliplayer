import { Component } from "@angular/core";
import { RouterLink } from "@angular/router";

@Component({
  selector: "app-home-page",
  standalone: true,
  imports: [RouterLink],
  template: `
    <section class="home-page">
      <p class="eyebrow" i18n="Home page eyebrow@@homeEyebrow">home</p>
      <h1 i18n="Home page title@@homeTitle">ライバーの歌配信を連続再生</h1>
      <p class="lede" i18n="Home page description@@homeDescription">
        歌配信のアーカイブから歌唱部分のみを連続的に再生できるサイトです。
      </p>

      <div class="cluster">
        <a
          class="button primary"
          [routerLink]="['/search']"
          i18n="Home primary action@@homePrimaryAction"
        >
          検索
        </a>
        <a
          class="button ghost"
          [routerLink]="['/settings']"
          i18n="Home secondary action@@homeSecondaryAction"
        >
          設定
        </a>
      </div>

      <div class="home-note">
        <p i18n="Home note about availability@@homeNote">
          YouTube の埋め込み制限や動画削除により、再生できない clip
          が含まれる場合があります。
        </p>
      </div>
    </section>
  `,
})
export class HomePageComponent {}
