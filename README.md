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
- `/json-beautify --indent 4 <json>`
- `/json-beautify --tabs <json>`
- `/json-uglify <json>`
- `/json-escape <json>`
- `/json-unescape <json>`

The extension has no Node.js runtime dependency. The command implementation is Rust compiled to WebAssembly by Zed. Number parsing enables arbitrary precision so large JSON numbers are preserved instead of being rewritten through floating-point notation.

## Local installation

In Zed, run `zed::InstallDevExtension` and select this directory.

Zed requires Rust installed via `rustup` when installing a dev extension. Homebrew Rust does not provide the WebAssembly target setup Zed expects, and Zed will fail with `failed to compile Rust extension`. Install Rust with `rustup` first, then add the WebAssembly target:

```sh
rustup target add wasm32-wasip1
```

On macOS, if Zed is launched from the Dock or Finder, it may not inherit your shell `PATH`. If `~/Library/Logs/Zed/Zed.log` contains `failed to run rustc: No such file or directory`, either fully quit Zed and start it from a terminal with `zed .`, or export a GUI session PATH before reopening Zed:

```sh
launchctl setenv PATH "$HOME/.cargo/bin:/usr/local/bin:/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin"
```

## Notes

The original VS Code plugin only transforms JSON values where JavaScript `typeof JSON.parse(text) === "object"`, which includes objects, arrays, and `null`, but excludes JSON strings, numbers, booleans, and invalid JSON. This port keeps that behavior.
