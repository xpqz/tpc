program SetAlgebra;
type TS = set of 0..31;
var a, b, c: TS; i, bad: Integer;
procedure Make(seed: Integer; var r: TS);
var k: Integer;
begin
  r := [];
  for k := 0 to 31 do
    if (k * seed + seed div 3) mod 5 < 2 then r := r + [k];
end;
begin
  bad := 0;
  for i := 1 to 12 do begin
    Make(i, a);
    Make(i + 5, b);
    Make(i * 3, c);
    if a + b <> b + a then bad := bad + 1;
    if a * b <> b * a then bad := bad + 1;
    if (a + b) + c <> a + (b + c) then bad := bad + 1;
    if a * (b + c) <> (a * b) + (a * c) then bad := bad + 1;
    if (a - b) * b <> [] then bad := bad + 1;
    if (a - b) + (a * b) <> a then bad := bad + 1;
    if not (a * b <= a) then bad := bad + 1;
    if not (a <= a + b) then bad := bad + 1;
  end;
  WriteLn(bad)
end.
