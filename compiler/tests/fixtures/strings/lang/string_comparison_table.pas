program StringComparisonTable;
var a, b: String; one: String[1]; c: Char;
procedure Show(x, y: String);
begin
  WriteLn(x, '|', y, ' ', x = y, ' ', x <> y, ' ', x < y, ' ', x > y, ' ', x <= y, ' ', x >= y)
end;
begin
  Show('', ''); Show('', 'a'); Show('a', ''); Show('a', 'a'); Show('a', 'b'); Show('b', 'a');
  Show('ab', 'a'); Show('a', 'ab'); Show('ab', 'b'); Show('B', 'a'); Show('a', 'B');
  Show('abc', 'abd'); Show('abc', 'ab'); Show(' ', ''); Show('Z', 'z'); Show('10', '9');
  one := 'a'; c := 'a';
  WriteLn(one = c, c = one, one < 'b', c < one, c <= one, 'a' = c, 'ab' > c, c < 'ab');
  a := 'q'; b := a;
  WriteLn(a = b, a = 'q', 'q' = a, a < 'r', a[1] = 'q', a[1] < b[1]);
  b[0] := Chr(0);
  WriteLn(b = '', Length(b));
  if a > b then WriteLn('longer is greater')
end.
