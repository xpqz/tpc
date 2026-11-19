program RealRel;
var i: Integer; l: LongInt; x, y: Real; b: Boolean;
procedure T(b: Boolean);
begin
  if b then Write('T') else Write('F')
end;
begin
  i := 3; l := -5; x := 3.0; y := 3.5;
  T(x = i); T(i = x); T(x <> i); T(y <> i); T(y > i); T(i < y); T(y >= x); T(x >= i); T(x <= i);
  T(y < x); T(l < y); T(l > x); T(-y < l); T(x = 3); T(x = 3.0); T(y = 3.5); T(0.5 + 0.25 = 0.75);
  WriteLn;
  b := x < y; WriteLn(b);
  b := (x < y) and (y < 4); WriteLn(b);
  b := (y > i) = (i < y); WriteLn(b);
  x := 0; T(x = 0); T(x <> 0); T(-x = 0); T(x < 1); T(x > -1);
  WriteLn;
  if x < y then WriteLn('lt') else WriteLn('ge');
  while x < 2 do x := x + 0.5;
  WriteLn(x:0:1);
  repeat y := y - 1 until y < 0;
  WriteLn(y:0:1)
end.
