program HexLiterals;
var
  i: Integer;
  w: Word;
  l: LongInt;
begin
  i := $7FFF;
  w := $FFFF;
  l := $7FFFFFFF;
  WriteLn(i, ' ', w, ' ', l);
  w := $ABCD;
  WriteLn(w);
  l := $10000;
  WriteLn(l);
  i := $0;
  WriteLn(i);
  l := $deadbe;
  WriteLn(l);
  WriteLn($10, ' ', $FF + 1)
end.
