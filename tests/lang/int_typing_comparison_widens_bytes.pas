program IntTypingComparisonWidensBytes;
var
  s: ShortInt;
  b: Byte;
  i: Integer;
  w: Word;
begin
  s := -1;
  b := 200;
  WriteLn(s < b, ' ', s > b, ' ', s = b);
  i := -1;
  w := 65535;
  WriteLn(i < w, ' ', i = w, ' ', i > w);
  b := 255;
  w := 255;
  WriteLn(b = w, ' ', b < w)
end.
