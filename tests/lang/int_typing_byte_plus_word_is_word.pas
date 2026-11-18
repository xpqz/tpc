program IntTypingBytePlusWordIsWord;
var
  b: Byte;
  w: Word;
begin
  b := 1;
  w := 65535;
  WriteLn(b + w);
  b := 0;
  w := 0;
  WriteLn(b - w - 1);
  b := 255;
  w := 65535;
  WriteLn(b * w)
end.
