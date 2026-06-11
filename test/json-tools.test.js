const assert = require("assert");
const {
  beautifyJson,
  escapeJson,
  isValid,
  uglifyJson,
  unescapeJson,
} = require("../bin/json-tools");

assert.strictEqual(isValid('{"a":1}'), true);
assert.strictEqual(isValid("[1,2]"), true);
assert.strictEqual(isValid("null"), true);
assert.strictEqual(isValid('"text"'), false);
assert.strictEqual(isValid("42"), false);
assert.strictEqual(isValid("{"), false);

assert.strictEqual(beautifyJson('{"a":1}'), '{\n  "a": 1\n}');
assert.strictEqual(uglifyJson('{\n  "a": 1\n}'), '{"a":1}');
assert.strictEqual(escapeJson('{"a":"b"}'), '{\\"a\\":\\"b\\"}');
assert.strictEqual(unescapeJson('{\\"a\\":\\"b\\"}'), '{"a":"b"}');

assert.strictEqual(beautifyJson("  nope  "), "nope");
assert.strictEqual(uglifyJson("  nope  "), "nope");
assert.strictEqual(escapeJson("  nope  "), "nope");
assert.strictEqual(unescapeJson("  \\q  "), "\\q");
