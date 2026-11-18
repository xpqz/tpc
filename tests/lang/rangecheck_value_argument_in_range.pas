program RangeArg;
{$R+}
type Small = 1..10;
var i: Integer;
procedure P(s: Small; b: Byte);
begin
  WriteLn(s, ' ', b);
end;
begin
  P(1, 0);
  P(10, 255);
  i := 5;
  P(i, i * 10);
  P(i + 5, 200 + i)
end.
