program ArgSite;
type Small = 1..10;
var i: Integer;
procedure P(s: Small);
begin
  WriteLn('in P ', s);
end;
begin
  i := 11;
  {$R-}
  P(i);
  {$R+}
  P(i);
  WriteLn('not reached')
end.
