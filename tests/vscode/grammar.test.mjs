// Checks the TextMate grammar without vscode-textmate: the JSON parses, every pattern compiles
// as a JavaScript regular expression (a leading (?i) becoming the i flag), and a small scanner
// that follows the TextMate rule (the earliest match wins, ties go to the earlier pattern)
// gives the expected scopes for single-line samples.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { root } from "./build.mjs";

const grammar = JSON.parse(fs.readFileSync(path.join(root, "vscode/syntaxes/tp5pascal.tmLanguage.json"), "utf8"));

const compile = (source) =>
  source.startsWith("(?i)") ? new RegExp(source.slice(4), "iy") : new RegExp(source, "y");

function resolve(rule) {
  if (!rule.include) return [rule];
  const name = rule.include.replace(/^#/, "");
  const target = grammar.repository[name];
  assert.ok(target, `unknown include ${rule.include}`);
  return target.patterns ? target.patterns.flatMap(resolve) : [target];
}

function everyRule(rules, visit) {
  for (const rule of rules) {
    visit(rule);
    if (rule.patterns) everyRule(rule.patterns, visit);
  }
}

// Scopes of `line`: a list of [text, scopeName] for the matched tokens only.
function scan(line) {
  const rules = grammar.patterns.flatMap(resolve);
  const tokens = [];
  let position = 0;
  while (position < line.length) {
    let best = null;
    for (const rule of rules) {
      const re = compile(rule.match ?? rule.begin);
      for (let at = position; at < line.length && (!best || at < best.at); at++) {
        re.lastIndex = at;
        const m = re.exec(line);
        if (m) {
          best = best && best.at <= at ? best : { at, m, rule };
          break;
        }
      }
    }
    if (!best) break;
    let end = best.at + best.m[0].length;
    if (best.rule.begin) {
      const inner = compile(best.rule.end);
      inner.lastIndex = end;
      const closing = (() => {
        for (let at = end; at <= line.length; at++) {
          inner.lastIndex = at;
          const m = inner.exec(line);
          if (m) return at + m[0].length;
        }
        return line.length;
      })();
      end = closing;
    }
    tokens.push([line.slice(best.at, end), best.rule.name]);
    position = Math.max(end, best.at + 1);
  }
  return tokens;
}

const scopeOf = (line, text) => {
  const token = scan(line).find(([t]) => t.toLowerCase() === text.toLowerCase());
  return token ? token[1] : undefined;
};

test("the grammar is a tp5pascal source grammar and every regular expression compiles", () => {
  assert.equal(grammar.scopeName, "source.tp5pascal");
  let count = 0;
  everyRule(Object.values(grammar.repository), (rule) => {
    for (const key of ["match", "begin", "end"]) {
      if (rule[key] !== undefined) {
        compile(rule[key]);
        count++;
      }
    }
  });
  assert.ok(count > 8, "the repository defines the token classes");
});

test("keywords are scoped in any case", () => {
  for (const word of ["begin", "BEGIN", "While", "repeat", "until", "downto", "case", "goto"]) {
    assert.match(scopeOf(`  ${word} x`, word) ?? "", /^keyword\.control/, word);
  }
  for (const word of ["var", "Procedure", "FUNCTION", "uses", "interface", "implementation", "forward"]) {
    assert.match(scopeOf(`${word} x`, word) ?? "", /^keyword\.other/, word);
  }
  for (const word of ["and", "or", "not", "xor", "div", "mod", "shl", "shr", "in"]) {
    assert.match(scopeOf(`a ${word} b`, word) ?? "", /^keyword\.operator\.word/, word);
  }
  assert.match(scopeOf("x: ARRAY [1..2] of integer", "ARRAY") ?? "", /^storage\.type/);
});

test("a keyword inside an identifier is not a keyword", () => {
  assert.equal(scopeOf("beginner := endx", "beginner"), undefined);
  assert.equal(scopeOf("my_begin := 1", "my_begin"), undefined);
});

test("standard types, routines and constants are support or constant scopes", () => {
  assert.match(scopeOf("var i: Integer;", "Integer") ?? "", /^support\.type/);
  assert.match(scopeOf("x: LongInt", "LongInt") ?? "", /^support\.type/);
  assert.match(scopeOf("  WriteLn(x)", "WriteLn") ?? "", /^support\.function/);
  assert.match(scopeOf("  x := Length(s)", "Length") ?? "", /^support\.function/);
  assert.match(scopeOf("p := nil", "nil") ?? "", /^constant\.language/);
  assert.match(scopeOf("b := True", "True") ?? "", /^constant\.language/);
});

test("brace comments, paren-star comments and directives are separate scopes", () => {
  assert.match(scopeOf("x := 1; { note }", "{ note }") ?? "", /^comment\.block/);
  assert.match(scopeOf("x := 1; (* note *)", "(* note *)") ?? "", /^comment\.block/);
  assert.match(scopeOf("{$R+}", "{$R+}") ?? "", /^meta\.preprocessor/);
  assert.match(scopeOf("{$I defs.inc}", "{$I defs.inc}") ?? "", /^meta\.preprocessor/);
  assert.match(scopeOf("(*$R-*)", "(*$R-*)") ?? "", /^meta\.preprocessor/);
  assert.equal(scopeOf("{ begin }", "begin"), undefined, "no token inside a comment");
});

test("strings use doubled quotes, and char literals use #", () => {
  assert.match(scopeOf("s := 'it''s'", "'it''s'") ?? "", /^string\.quoted\.single/);
  assert.match(scopeOf("s := ''", "''") ?? "", /^string\.quoted\.single/);
  assert.match(scopeOf("c := #65", "#65") ?? "", /^constant\.character/);
  assert.match(scopeOf("c := #$1B", "#$1B") ?? "", /^constant\.character/);
  assert.equal(scopeOf("s := 'begin'", "begin"), undefined, "no keyword inside a string");
});

test("numbers: decimal, hex, real with exponent", () => {
  assert.match(scopeOf("x := 123", "123") ?? "", /^constant\.numeric/);
  assert.match(scopeOf("x := $FF00", "$FF00") ?? "", /^constant\.numeric/);
  assert.match(scopeOf("x := $ff", "$ff") ?? "", /^constant\.numeric/);
  assert.match(scopeOf("x := 3.14", "3.14") ?? "", /^constant\.numeric/);
  assert.match(scopeOf("x := 1.5E-3", "1.5E-3") ?? "", /^constant\.numeric/);
  assert.equal(scopeOf("a[1..2]", "1.") , undefined, "a subrange is not a real");
});
