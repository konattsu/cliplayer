import type { Locale } from "../../shared/contracts/common.schema";

export const DEFAULT_LOCALE: Locale = "ja";
export const SUPPORTED_LOCALES: readonly Locale[] = ["ja", "en"];

export function isLocale(value: string | null | undefined): value is Locale {
  return value === "ja" || value === "en";
}

export function localeFromPath(pathname: string): Locale {
  const segment = pathname.split("/").filter(Boolean)[0] ?? null;
  return isLocale(segment) ? segment : DEFAULT_LOCALE;
}

export function switchLocaleUrl(
  targetLocale: Locale,
  currentPathname: string,
  search = "",
  hash = "",
): string {
  const normalizedPath = currentPathname.startsWith("/")
    ? currentPathname
    : `/${currentPathname}`;
  const pathWithoutLocale = normalizedPath.replace(/^\/(?:ja|en)(?=\/|$)/, "");
  const normalizedSuffix =
    pathWithoutLocale.length === 0 ? "/" : pathWithoutLocale;
  return `/${targetLocale}${normalizedSuffix}${search}${hash}`;
}
