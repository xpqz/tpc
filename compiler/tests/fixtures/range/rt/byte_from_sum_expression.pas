program ByteSum;
{$R+}
var b, c: Byte;
begin
  b := 250;
  c := 5;
  b := b + c;
  WriteLn(b);
  c := 6;
  b := b + c;
  WriteLn('not reached')
end.
