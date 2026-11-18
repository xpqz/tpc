program IntTypingShortIntPlusShortInt;
var a, b: ShortInt;
begin
  a := 100;
  b := 100;
  WriteLn(a + b);
  a := -100;
  b := -100;
  WriteLn(a + b);
  a := 127;
  b := -128;
  WriteLn(a - b)
end.
