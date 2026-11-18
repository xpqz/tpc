program IntTypingShlKeepsOperandType;
var
  b: Byte;
  w: Word;
  i: Integer;
begin
  b := 200;
  WriteLn(b shl 1);
  b := 1;
  WriteLn(b shl 8);
  w := $8001;
  WriteLn(w shl 1);
  w := $FFFF;
  WriteLn(w shl 4);
  i := 16384;
  WriteLn(i shl 1);
  i := 16384;
  WriteLn(i shl 2)
end.
