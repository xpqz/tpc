program RangeShort;
{$R+}
var s: ShortInt; i: Integer;
begin
  WriteLn('before');
  i := 128;
  s := i;
  WriteLn(s);
  WriteLn('not reached')
end.
