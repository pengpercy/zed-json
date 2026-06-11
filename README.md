# JSON Tools for Zed

This is a Zed-oriented port of the VS Code `vscode-json` extension. It implements the same JSON operations:

- validate
- beautify
- uglify
- escape
- unescape

## Zed support status

Zed extensions currently do not expose the VS Code-style editor command API needed to replace the active selection or whole buffer directly. This extension therefore provides Zed Assistant slash commands plus a local CLI with the same transformation logic.

Slash commands:

- `/json-validate <json>`
- `/json-beautify <json>`
- `/json-uglify <json>`
- `/json-escape <json>`
- `/json-unescape <json>`

CLI usage:

```sh
node bin/json-tools.js validate '{"a":1}'
node bin/json-tools.js beautify '{"a":1}'
node bin/json-tools.js uglify '{ "a": 1 }'
node bin/json-tools.js escape '{"a":"b"}'
node bin/json-tools.js unescape '{\"a\":\"b\"}'
```

If no text argument is supplied, the CLI reads from stdin:

```sh
node bin/json-tools.js beautify < package.json
```

## Local installation

In Zed, run `zed::InstallDevExtension` and select this directory.

## Notes

The original VS Code plugin only transforms JSON values where JavaScript `typeof JSON.parse(text) === "object"`, which includes objects, arrays, and `null`, but excludes JSON strings, numbers, booleans, and invalid JSON. This port keeps that behavior.
