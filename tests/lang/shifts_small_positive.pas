program ShiftsSmallPositive;
var
  i: Integer;
  b: Byte;
begin
  i := 3;
  i := i shl 5;
  WriteLn(i);
  i := 1000;
  WriteLn(i shr 2, ' ', i shr 9);
  b := 3;
  b := b shl 2;
  WriteLn(b);
  b := 200;
  WriteLn(b shr 3, ' ', b shr 7)
end.
