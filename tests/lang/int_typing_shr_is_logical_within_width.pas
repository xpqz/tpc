program IntTypingShrIsLogicalWithinWidth;
var
  b: Byte;
  s: ShortInt;
  i: Integer;
  l: LongInt;
begin
  b := 200;
  WriteLn(b shr 1);
  s := -128;
  WriteLn(s shr 1);
  s := -1;
  WriteLn(s shr 7);
  i := -32768;
  WriteLn(i shr 1);
  i := -1;
  WriteLn(i shr 12);
  l := -1;
  WriteLn(l shr 28);
  l := -2147483647 - 1;
  WriteLn(l shr 1)
end.
