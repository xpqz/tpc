program SetDynamic;
var s: set of 0..30; a, b, k, n: Integer;
begin
  for a := 0 to 6 do begin
    b := 6 - a;
    s := [a..b];
    n := 0;
    for k := 0 to 30 do
      if k in s then n := n + 1;
    Write(n, ' ');
  end;
  WriteLn;
  a := 10;
  s := [a, a * 2, a * 3, a - 10];
  for k := 0 to 30 do
    if k in s then Write(k, ' ');
  WriteLn;
  s := [a + 1..a + 3] + [a - 1..a - 3];
  for k := 0 to 30 do
    if k in s then Write(k, ' ');
  WriteLn
end.
