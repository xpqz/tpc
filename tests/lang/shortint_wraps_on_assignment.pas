program ShortIntWrapsOnAssignment;
var s: ShortInt;
begin
  s := 127;
  s := s + 1;
  WriteLn(s);
  s := -128;
  s := s - 1;
  WriteLn(s);
  s := 100;
  s := s + s;
  WriteLn(s);
  s := -100;
  s := s + s;
  WriteLn(s);
  s := 64;
  s := s * 2;
  WriteLn(s)
end.
