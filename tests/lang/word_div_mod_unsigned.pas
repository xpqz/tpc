program WordDivModUnsigned;
var w, v: Word;
begin
  w := 65535;
  v := 2;
  WriteLn(w div v, ' ', w mod v);
  v := 256;
  WriteLn(w div v, ' ', w mod v);
  w := 40000;
  v := 7;
  WriteLn(w div v, ' ', w mod v);
  w := 32768;
  WriteLn(w div 3, ' ', w mod 3)
end.
