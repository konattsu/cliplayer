import z from "zod";

export const ArtistIdSchema = z.string().nonempty();
export type ArtistId = z.infer<typeof ArtistIdSchema>;
