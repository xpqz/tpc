program IntTypingIntegerWordCommonLongInt;
var
  i: Integer;
  w: Word;
begin
  i := -1;
  w := 65535;
  WriteLn(i + w);
  i := 0;
  w := 1;
  WriteLn(i - w);
  i := -32768;
  w := 65535;
  WriteLn(i - w)
end.
