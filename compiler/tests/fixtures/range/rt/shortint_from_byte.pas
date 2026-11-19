program ShortByte;
{$R+}
var s: ShortInt; b: Byte;
begin
  b := 127;
  s := b;
  WriteLn(s);
  b := 128;
  s := b;
  WriteLn('not reached')
end.
