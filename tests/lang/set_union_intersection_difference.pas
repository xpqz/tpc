program SetOps;
type TS = set of 0..20;
var a, b, c: TS;
procedure Show(s: TS);
var k: Integer;
begin
  for k := 0 to 20 do
    if k in s then Write(k, ' ');
  WriteLn;
end;
begin
  a := [1..8];
  b := [5..12];
  Show(a + b);
  Show(a * b);
  Show(a - b);
  Show(b - a);
  c := a + [15, 20];
  Show(c);
  Show(c - [1..7]);
  Show(a * []);
  Show(a + []);
  Show([] - a);
  Show((a + b) - (a * b));
  Show(a * b * [6, 7, 8, 9]);
  Show(a + b + [0] - [5]);
  c := a;
  c := c + [9];
  Show(a);
  Show(c)
end.
