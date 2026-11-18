program BitwiseOperatorsWordByte;
var
  w: Word;
  b: Byte;
begin
  w := $0F0F;
  w := not w;
  WriteLn(w);
  w := $FFFF;
  WriteLn(w and $00FF, ' ', w xor $FFFF, ' ', w or 0);
  b := 5;
  b := not b;
  WriteLn(b);
  b := $F0;
  WriteLn(b and $3C, ' ', b or $0F, ' ', b xor $FF);
  b := 0;
  b := not b;
  WriteLn(b)
end.
