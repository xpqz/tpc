program IntTypingLongIntPlusWrapsAt32Bits;
var l: LongInt;
begin
  l := 2147483647;
  WriteLn(l + 1);
  l := -2147483647 - 1;
  WriteLn(l - 1);
  l := 65536;
  WriteLn(l * l);
  l := 2000000000;
  WriteLn((l + l) div 2)
end.
