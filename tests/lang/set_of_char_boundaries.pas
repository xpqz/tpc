program SetCharBounds;
var s: set of Char; c: Char; n: Integer;
begin
  s := [#0, #255, 'A'];
  WriteLn(#0 in s, ' ', #255 in s, ' ', 'A' in s, ' ', 'B' in s, ' ', #1 in s, ' ', #254 in s);
  s := [#0..#255];
  n := 0;
  for c := #0 to #255 do
    if c in s then n := n + 1;
  WriteLn(n);
  s := s - ['a'..'z'];
  n := 0;
  for c := #0 to #255 do
    if c in s then n := n + 1;
  WriteLn(n);
  s := [#254..#255, #0..#1];
  n := 0;
  for c := #0 to #255 do
    if c in s then n := n + Ord(c);
  WriteLn(n)
end.
