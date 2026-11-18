program IntTypingNotKeepsOperandType;
var
  b: Byte;
  w: Word;
  s: ShortInt;
  i: Integer;
begin
  b := 0;
  WriteLn(not b);
  b := 170;
  WriteLn(not b);
  w := 0;
  WriteLn(not w);
  s := 0;
  WriteLn(not s);
  i := 0;
  WriteLn(not i)
end.
