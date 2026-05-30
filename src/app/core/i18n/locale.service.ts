import { computed, inject, Injectable, LOCALE_ID } from "@angular/core";

import type { Locale } from "../../shared/contracts/common.schema";

@Injectable({ providedIn: "root" })
export class LocaleService {
  private readonly localeId = inject(LOCALE_ID);

  readonly locale = computed<Locale>(() => {
    return this.localeId.startsWith("en") ? "en" : "ja";
  });
}
