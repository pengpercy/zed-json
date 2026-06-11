# JSON Tools for Zed

This is a Zed-oriented port of the VS Code `vscode-json` extension. It implements the same JSON operations:

- validate
- beautify
- uglify
- escape
- unescape

It also includes a small set of JSON snippets so the extension provides a Zed marketplace-supported capability.

## Zed support status

Zed extensions currently do not expose the VS Code-style editor command API needed to replace the active selection or whole buffer directly. This extension therefore provides Zed Assistant slash commands with the same JSON transformation logic.

Slash commands:

- `/json-validate <json>`
- `/json-beautify <json>`
- `/json-uglify <json>`
- `/json-escape <json>`
- `/json-unescape <json>`

The extension has no Node.js runtime dependency. The command implementation is Rust compiled to WebAssembly by Zed.

## Local installation

In Zed, run `zed::InstallDevExtension` and select this directory.

## Notes

The original VS Code plugin only transforms JSON values where JavaScript `typeof JSON.parse(text) === "object"`, which includes objects, arrays, and `null`, but excludes JSON strings, numbers, booleans, and invalid JSON. This port keeps that behavior.
