program ArgOff;
type Small = 1..10;
var i: Integer;
{$R+}
procedure P(s: Small);
begin
  WriteLn('in P ', s);
end;
{$R-}
begin
  i := 11;
  P(i);
  i := 300;
  P(i)
end.
