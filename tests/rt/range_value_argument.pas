program RangeArgument;
{$R+}
type Small = 1..10;
procedure P(s: Small);
begin
  WriteLn('in P ', s);
end;
var i: Integer;
begin
  WriteLn('before');
  i := 10;
  P(i);
  i := 11;
  P(i);
  WriteLn('not reached')
end.
