import { cp, mkdir, rm, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn } from "node:child_process";

const rootDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const distDir = resolve(rootDir, "dist/site");

await rm(distDir, { recursive: true, force: true });
await run("pnpm", ["build:ja"]);
await flattenBrowserOutput("ja");
await run("pnpm", ["build:en"]);
await flattenBrowserOutput("en");
await mkdir(distDir, { recursive: true });
await writeFile(
  resolve(distDir, "index.html"),
  `<!doctype html>
<html lang="ja">
  <head>
    <meta charset="utf-8" />
    <meta http-equiv="refresh" content="0; url=/ja/" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>cliplayer</title>
  </head>
  <body></body>
</html>
`,
  "utf8",
);

function run(command, args) {
  return new Promise((resolvePromise, rejectPromise) => {
    const child = spawn(command, args, {
      cwd: rootDir,
      stdio: "inherit",
      shell: false,
    });

    child.on("exit", (code) => {
      if (code === 0) {
        resolvePromise();
        return;
      }

      rejectPromise(
        new Error(`${command} ${args.join(" ")} exited with code ${code}`),
      );
    });
    child.on("error", rejectPromise);
  });
}

async function flattenBrowserOutput(locale) {
  const localeDir = resolve(distDir, locale);
  const browserDir = resolve(localeDir, "browser");
  await cp(browserDir, localeDir, { recursive: true, force: true });
  await rm(browserDir, { recursive: true, force: true });
}
