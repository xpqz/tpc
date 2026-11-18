program WordWrapsOnAssignment;
var w: Word;
begin
  w := 65535;
  w := w + 1;
  WriteLn(w);
  w := 0;
  w := w - 1;
  WriteLn(w);
  w := 40000;
  w := w + w;
  WriteLn(w);
  w := 300;
  w := w * 300;
  WriteLn(w)
end.
