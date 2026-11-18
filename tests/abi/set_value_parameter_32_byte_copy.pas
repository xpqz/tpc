program SetValueParam;
type CS = set of Char;
function Count(s: CS): Integer;
var c: Char; n: Integer;
begin
  n := 0;
  for c := #0 to #255 do
    if c in s then n := n + 1;
  Count := n;
end;
procedure Grow(s: CS; var n: Integer);
begin
  s := s + ['a'..'z'];
  n := Count(s);
end;
function Both(a, b: CS; x: Integer): Integer;
begin
  Both := Count(a * b) * 1000 + Count(a + b) * 10 + x;
end;
var s: CS; n, k: Integer;
begin
  s := ['0'..'9'];
  Grow(s, n);
  WriteLn(n, ' ', Count(s));
  WriteLn(Both(['a'..'m'], ['h'..'z'], 7));
  for k := 1 to 100 do n := Count(s) + Both(s, [], k);
  WriteLn(n)
end.
