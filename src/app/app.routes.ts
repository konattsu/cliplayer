import type { Routes } from "@angular/router";

export const appRoutes: Routes = [
  {
    path: "",
    loadComponent: () =>
      import("./features/home/home-page.component").then(
        (module) => module.HomePageComponent,
      ),
  },
  {
    path: "search",
    loadComponent: () =>
      import("./features/search/search-page.component").then(
        (module) => module.SearchPageComponent,
      ),
  },
  {
    path: "play",
    loadComponent: () =>
      import("./features/play/play-page.component").then(
        (module) => module.PlayPageComponent,
      ),
  },
  {
    path: "clip/:clipUuid",
    loadComponent: () =>
      import("./features/play/play-page.component").then(
        (module) => module.PlayPageComponent,
      ),
  },
  {
    path: "settings",
    loadComponent: () =>
      import("./features/settings/settings-page.component").then(
        (module) => module.SettingsPageComponent,
      ),
  },
  {
    path: "**",
    loadComponent: () =>
      import("./features/not-found/not-found-page.component").then(
        (module) => module.NotFoundPageComponent,
      ),
  },
];
