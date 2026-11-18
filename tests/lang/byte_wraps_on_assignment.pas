program ByteWrapsOnAssignment;
var b: Byte;
begin
  b := 255;
  b := b + 1;
  WriteLn(b);
  b := 0;
  b := b - 1;
  WriteLn(b);
  b := 200;
  b := b + b;
  WriteLn(b);
  b := 16;
  b := b * 17;
  WriteLn(b);
  b := 255;
  b := b * 255;
  WriteLn(b)
end.
