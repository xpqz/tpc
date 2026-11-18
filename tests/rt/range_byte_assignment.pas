program RangeByte;
{$R+}
var b: Byte; i: Integer;
begin
  WriteLn('before');
  i := 256;
  b := i;
  WriteLn(b);
  WriteLn('not reached')
end.
