import { z } from "zod";

import { LocaleSchema } from "../../shared/contracts/common.schema";

export const ThemePreferenceSchema = z.enum(["light", "dark", "system"]);
export const AmbientColorHexSchema = z
  .string()
  .regex(/^#[0-9a-fA-F]{6}$/)
  .nullable();

export const UserPreferencesSchema = z.object({
  version: z.literal(1),
  locale: LocaleSchema,
  theme: ThemePreferenceSchema,
  ambientColorHex: AmbientColorHexSchema,
  searchMode: z.literal("basic"),
  filterGraduatedArtists: z.enum(["strict", "loose", "disabled"]),
});

export type ThemePreference = z.infer<typeof ThemePreferenceSchema>;
export type UserPreferences = z.infer<typeof UserPreferencesSchema>;
