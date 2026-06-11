#!/usr/bin/env node

const fs = require("fs");

function usage() {
  return [
    "Usage: json-tools <validate|beautify|uglify|escape|unescape> [text]",
    "",
    "If text is omitted, input is read from stdin.",
  ].join("\n");
}

function trimJsonText(text) {
  return text.trim().replace(/(?:^[\n\t\r]|[\n\t\r]$)/g, "");
}

function parseValidJson(text) {
  const trimmed = trimJsonText(text);
  try {
    const value = JSON.parse(trimmed);
    return value !== undefined &&
      (value === null || Array.isArray(value) || typeof value === "object")
      ? value
      : undefined;
  } catch {
    return undefined;
  }
}

function isValid(text) {
  return parseValidJson(text) !== undefined;
}

function escapeJson(text) {
  const trimmed = trimJsonText(text);
  return isValid(trimmed)
    ? JSON.stringify(trimmed).replace(/^"/g, "").replace(/"$/g, "")
    : trimmed;
}

function unescapeJson(text) {
  const trimmed = trimJsonText(text);
  let formatted = trimmed;

  try {
    if (!trimmed.startsWith('"')) {
      formatted = `"${formatted}`;
    }
    if (!trimmed.endsWith('"')) {
      formatted = `${formatted}"`;
    }
    return JSON.parse(formatted);
  } catch {
    return trimmed;
  }
}

function beautifyJson(text, indent = 2) {
  const trimmed = trimJsonText(text);
  const value = parseValidJson(trimmed);
  return value !== undefined ? JSON.stringify(value, null, indent) : trimmed;
}

function uglifyJson(text) {
  const trimmed = trimJsonText(text);
  const value = parseValidJson(trimmed);
  return value !== undefined ? JSON.stringify(value) : trimmed;
}

function readInput(args) {
  if (args.length > 0) {
    return args.join(" ");
  }
  return fs.readFileSync(0, "utf8");
}

function main(argv) {
  const [command, ...args] = argv;
  if (!command || command === "-h" || command === "--help") {
    console.log(usage());
    return 0;
  }

  const input = readInput(args);
  switch (command) {
    case "validate":
      console.log(isValid(input) ? "Valid JSON" : "Invalid JSON");
      return isValid(input) ? 0 : 1;
    case "beautify":
      process.stdout.write(beautifyJson(input));
      return 0;
    case "uglify":
      process.stdout.write(uglifyJson(input));
      return 0;
    case "escape":
      process.stdout.write(escapeJson(input));
      return 0;
    case "unescape":
      process.stdout.write(unescapeJson(input));
      return 0;
    default:
      console.error(usage());
      return 2;
  }
}

if (require.main === module) {
  process.exitCode = main(process.argv.slice(2));
}

module.exports = {
  beautifyJson,
  escapeJson,
  isValid,
  uglifyJson,
  unescapeJson,
};
