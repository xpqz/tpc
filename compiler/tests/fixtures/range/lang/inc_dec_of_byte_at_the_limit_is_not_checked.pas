program IncDec;
{$R+}
type Small = 1..10;
var b: Byte; s: Small; i: Integer;
begin
  b := 255;
  Inc(b);
  WriteLn(b);
  Dec(b);
  WriteLn(b);
  i := 3;
  b := 254;
  Inc(b, i);
  WriteLn(b);
  b := 1;
  Dec(b, i);
  WriteLn(b);
  s := 5;
  Inc(s);
  Dec(s, 2);
  WriteLn(s)
end.
