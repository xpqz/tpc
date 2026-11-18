program IntTypingShortIntWithWordIsLongInt;
var
  s: ShortInt;
  w: Word;
begin
  s := -100;
  w := 0;
  WriteLn(s + w);
  s := -1;
  w := 5;
  WriteLn(s - w);
  s := 127;
  w := 65535;
  WriteLn(s + w)
end.
