import importPlugin from "eslint-plugin-import";
import jsxA11y from "eslint-plugin-jsx-a11y";
import prettierConfig from "eslint-config-prettier/flat";
import prettierPlugin from "eslint-plugin-prettier";
import unusedImports from "eslint-plugin-unused-imports";
import tseslintParser from "@typescript-eslint/parser";
import tseslintPlugin from "@typescript-eslint/eslint-plugin";
import angular from "angular-eslint";

// ref: https://eslint.org/docs/latest/use/configure/ignore#including-gitignore-files
import { includeIgnoreFile } from "@eslint/compat";
import { fileURLToPath } from "url";

const gitignorePath = fileURLToPath(new URL("./.gitignore", import.meta.url));

// angular-eslintはextendsを使う前提で作られている.
// しかし, このプロジェクトはflatを使うのでextendsできない. そのためなんか汚い処理が必要らしい
// Normalize an angular-eslint exported config (object or array) into an array
// of flat-config objects and scope them to the given `files` glob.
function normalizeAndScopeAngularConfigs(
  cfg: any,
  files: string[] = ["src/**/*.html"],
) {
  if (!cfg) return [];
  const arr = Array.isArray(cfg) ? cfg : [cfg];
  return arr.map((c) => {
    if (!c || typeof c !== "object") return { files };
    return { ...c, files };
  });
}

export default [
  includeIgnoreFile(gitignorePath),
  {
    languageOptions: {
      globals: {
        window: "readonly",
        document: "readonly",
        console: "readonly",
        setTimeout: "readonly",
        clearTimeout: "readonly",
        setInterval: "readonly",
        clearInterval: "readonly",
        process: "readonly",
        module: "readonly",
        require: "readonly",
        alert: "readonly",
      },
      ecmaVersion: "latest",
      sourceType: "module",
    },
  },
  prettierConfig,

  // For test
  {
    files: ["**/*.spec.ts", "**/*.spec.tsx", "tests/**/*.ts"],
    languageOptions: {
      globals: {
        describe: "readonly",
        it: "readonly",
        expect: "readonly",
        beforeEach: "readonly",
        afterEach: "readonly",
        spyOn: "readonly",
      },
    },
  },

  // typescript
  {
    files: ["src/**/*.ts"],
    languageOptions: {
      parser: tseslintParser,
      parserOptions: {
        project: ["./tsconfig.eslint.json"],
        tsconfigRootDir: process.cwd(),
        ecmaVersion: "latest",
        sourceType: "module",
      },
    },
    plugins: {
      "@typescript-eslint": tseslintPlugin,
      prettier: prettierPlugin,
      "unused-imports": unusedImports,
      "jsx-a11y": jsxA11y,
      import: importPlugin,
    },
    rules: {
      "@typescript-eslint/consistent-type-definitions": ["error", "interface"],
      "@typescript-eslint/explicit-function-return-type": ["error"],
      "@typescript-eslint/no-explicit-any": "error",
      // ref: https://stackoverflow.com/questions/64052318#answer-64067915
      // それでもなぜかwarnでる. またdisable-next-lineしたらwarnになるので別の何かがwarn出してるっぽい
      "no-unused-vars": "off",
      "@typescript-eslint/no-unused-vars": [
        "error",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
          destructuredArrayIgnorePattern: "^_",
          // ignoreRestSiblings: true,
        },
      ],
      "@typescript-eslint/ban-ts-comment": [
        "error",
        { "ts-expect-error": false },
      ],
      "@typescript-eslint/no-floating-promises": "error",
      "@typescript-eslint/strict-boolean-expressions": [
        "warn",
        {
          allowString: false,
          allowNumber: false,
          allowNullableObject: false,
          allowNullableBoolean: false,
          allowNullableString: false,
          allowNullableNumber: false,
        },
      ],
      "@typescript-eslint/consistent-type-imports": [
        "error",
        { prefer: "type-imports" },
      ],
      "@typescript-eslint/restrict-plus-operands": [
        "error",
        {
          allowBoolean: false,
          allowNullish: false,
          allowNumberAndString: false,
          allowRegExp: false,
          allowAny: false,
        },
      ],
      "@typescript-eslint/switch-exhaustiveness-check": "error",
      "@typescript-eslint/method-signature-style": "error",

      // import
      "import/order": [
        "error",
        {
          groups: [
            "builtin",
            "external",
            "internal",
            "parent",
            "sibling",
            "index",
            "object",
            "type",
          ],
          pathGroups: [
            {
              pattern: "@src/**",
              group: "parent",
              position: "before",
            },
          ],
          pathGroupsExcludedImportTypes: ["builtin"],
          alphabetize: {
            order: "asc",
          },
          "newlines-between": "always",
        },
      ],
      "prettier/prettier": "error",

      // others
      "no-implicit-coercion": "error",
      "prefer-template": "error",
    },
  },

  // expand angular-eslint template configs and scope them to src HTML files
  ...normalizeAndScopeAngularConfigs(angular.configs.templateRecommended),
  ...normalizeAndScopeAngularConfigs(angular.configs.templateAccessibility),
];
