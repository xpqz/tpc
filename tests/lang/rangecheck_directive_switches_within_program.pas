program RangeSwitch;
type Small = 1..10;
var s: Small; b: Byte; i: Integer;
procedure Checked(n: Integer);
{$R+}
var t: Small;
begin
  t := n;
  WriteLn('checked ', t);
end;
{$R-}
procedure Unchecked(n: Integer);
var t: Byte;
begin
  t := n;
  WriteLn('unchecked ', t);
end;
begin
  Checked(10);
  Unchecked(257);
  i := 258;
  b := i;
  WriteLn(b)
end.
