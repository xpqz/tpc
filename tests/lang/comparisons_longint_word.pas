program ComparisonsLongIntWord;
var
  l, m: LongInt;
  w, v: Word;
begin
  l := 2147483647;
  m := -2147483647;
  WriteLn(l > m, ' ', l < m, ' ', m < 0, ' ', l = l);
  w := 65535;
  v := 1;
  WriteLn(w > v, ' ', w < v, ' ', w >= 65535, ' ', v <= 0);
  w := 40000;
  WriteLn(w > 32767, ' ', w < 50000)
end.
