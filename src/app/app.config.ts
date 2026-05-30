import { provideRouter } from "@angular/router";

import { appRoutes } from "./app.routes";

import type { ApplicationConfig } from "@angular/core";

export const appConfig: ApplicationConfig = {
  providers: [provideRouter(appRoutes)],
};
