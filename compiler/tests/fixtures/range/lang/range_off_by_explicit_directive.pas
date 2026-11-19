program ExplicitOff;
{$R-}
type Small = 1..10;
var s: Small; b: Byte; w: Word; i: Integer; sh: ShortInt;
    a: array[1..3] of Integer;
begin
  i := 70000 - 65536 + 100;
  b := i;
  WriteLn(b);
  i := -1;
  w := i;
  WriteLn(w);
  i := 200;
  sh := i;
  WriteLn(sh);
  i := 11;
  s := i;
  WriteLn(s);
  a[1] := 5;
  WriteLn(a[1])
end.
