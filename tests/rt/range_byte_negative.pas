program RangeByteNeg;
{$R+}
var b: Byte; i: Integer;
begin
  WriteLn('before');
  i := -1;
  b := i;
  WriteLn(b);
  WriteLn('not reached')
end.
