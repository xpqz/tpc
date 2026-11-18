program RangeOff;
type Small = 1..10;
var s: Small; b: Byte; i: Integer; sh: ShortInt;
begin
  i := 300;
  b := i;
  WriteLn(b);
  i := -1;
  b := i;
  WriteLn(b);
  i := 200;
  sh := i;
  WriteLn(sh);
  i := 5;
  s := i;
  WriteLn(s);
  i := 7;
  b := i * 100;
  WriteLn(b)
end.
