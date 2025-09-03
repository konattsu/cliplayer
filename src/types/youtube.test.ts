import { describe, it, expect } from "vitest";

import { VideoIdSchema, ChannelIdSchema } from "./youtube";

describe("VideoIdSchema", () => {
  it("accepts valid YouTube video IDs", () => {
    expect(() => VideoIdSchema.parse("dQw4w9WgXcQ")).not.toThrow();
    expect(() => VideoIdSchema.parse("a1b2c3d4e5F")).not.toThrow();
    expect(() => VideoIdSchema.parse("A-B_C_D-E1_")).not.toThrow();
    expect(() => VideoIdSchema.parse("00000000000")).not.toThrow();
    expect(() => VideoIdSchema.parse("-----------")).not.toThrow();
  });

  it("rejects invalid YouTube video IDs", () => {
    expect(() => VideoIdSchema.parse("short")).toThrow();
    expect(() => VideoIdSchema.parse("TooLongTooLongTooLong")).toThrow();
    expect(() => VideoIdSchema.parse("invalid!@#")).toThrow();
    expect(() => VideoIdSchema.parse("")).toThrow();
  });
});

describe("ChannelIdSchema", () => {
  it("accepts valid YouTube channel IDs", () => {
    expect(() =>
      ChannelIdSchema.parse("UCabcdefghijklmno1234567"),
    ).not.toThrow();
    expect(() =>
      ChannelIdSchema.parse("UC1234567890abcdefGHIJKL"),
    ).not.toThrow();
    expect(() =>
      ChannelIdSchema.parse("UCa-b_c-d_e-f_g-h_i-j_k-"),
    ).not.toThrow();
  });

  it("rejects invalid YouTube channel IDs", () => {
    expect(() => ChannelIdSchema.parse("abcdefghijklmno123456789")).toThrow(); // UCなし
    expect(() => ChannelIdSchema.parse("UCshort")).toThrow();
    expect(() =>
      ChannelIdSchema.parse("UCTooLongTooLongTooLongTooLongTooLong"),
    ).toThrow();
    expect(() => ChannelIdSchema.parse("UCinvalid!@#")).toThrow();
    expect(() => ChannelIdSchema.parse("")).toThrow();
  });
});
