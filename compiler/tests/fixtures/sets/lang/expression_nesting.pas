program SetExprNesting;
type CS = set of Char; BS = set of Byte;
var a, b, c: BS; x: Integer; letters: CS; ch: Char;
function Count(s: BS): Integer;
var k, n: Integer;
begin
  n := 0;
  for k := 0 to 255 do
    if k in s then n := n + 1;
  Count := n
end;
function Twice(i: Integer): Integer;
begin
  Twice := i * 2
end;
begin
  a := [1..10]; b := [5..15]; c := [8, 9, 100];
  WriteLn(Count(a + b), ' ', Count(a * b), ' ', Count(a - b), ' ', Count(b - a));
  WriteLn(Count((a + b) * (c + [1])), ' ', Count(a - (b - c)), ' ', Count(a + b - c * b));
  WriteLn(Count([1, 2, 3] + a), ' ', Count([]), ' ', Count([0..255]), ' ', Count([255]));
  x := 9;
  WriteLn(x in a + b, ' ', x in a * c, ' ', 100 in c - a, ' ', (x in a) = (x in c));
  WriteLn(Twice(4) in a, ' ', Twice(8) in b, ' ', Twice(Twice(2)) in [1..3] + [8]);
  a := [Twice(1), Twice(2), Twice(3)..Twice(4)];
  WriteLn(Count(a), ' ', 2 in a, ' ', 4 in a, ' ', 6 in a, ' ', 7 in a, ' ', 8 in a);
  letters := ['a'..'e'] + ['x'] - ['c'];
  for ch := 'a' to 'z' do if ch in letters then Write(ch);
  WriteLn;
  WriteLn(['a'] <= letters, ' ', letters >= ['a', 'b'], ' ', letters = ['a', 'b', 'd', 'e', 'x']);
  WriteLn(Count(a + a), ' ', Count(a * a), ' ', Count(a - a))
end.
