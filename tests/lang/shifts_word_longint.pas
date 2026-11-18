program ShiftsWordLongInt;
var
  w: Word;
  l: LongInt;
begin
  w := 1;
  w := w shl 4;
  WriteLn(w);
  w := $8000;
  w := w shl 1;
  WriteLn(w);
  w := $8000;
  WriteLn(w shr 15, ' ', w shr 1, ' ', w shr 0);
  w := $FFFF;
  w := w shl 8;
  WriteLn(w);
  l := 1;
  l := l shl 31;
  WriteLn(l);
  l := 1;
  l := l shl 20;
  WriteLn(l);
  l := 1000000;
  WriteLn(l shr 3, ' ', l shr 10)
end.
