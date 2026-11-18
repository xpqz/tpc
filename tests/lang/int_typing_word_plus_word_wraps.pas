program IntTypingWordPlusWordWraps;
var w, v: Word;
begin
  w := 65535;
  v := 1;
  WriteLn(w + v);
  WriteLn(v - w);
  w := 40000;
  WriteLn(w + w);
  WriteLn((w + w) div 2)
end.
