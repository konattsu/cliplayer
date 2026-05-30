import { Component } from "@angular/core";
import { RouterLink } from "@angular/router";

@Component({
  selector: "app-not-found-page",
  standalone: true,
  imports: [RouterLink],
  template: `
    <section class="empty-state">
      <p class="eyebrow" i18n="Not found eyebrow@@notFoundEyebrow">not found</p>
      <h1 i18n="Not found title@@notFoundTitle">
        ページまたは clip が見つかりません
      </h1>
      <p class="muted" i18n="Not found description@@notFoundDescription">
        URL を確認するか、検索ページから clip を探してください。
      </p>
      <a
        class="button primary"
        [routerLink]="['/search']"
        i18n="Not found action@@notFoundAction"
      >
        検索を開く
      </a>
    </section>
  `,
})
export class NotFoundPageComponent {}
