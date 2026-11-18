program StringToChar;
var s: String; c: Char;
begin
  s := 'q';
  c := s[1];
  WriteLn(c);
  s := 'abc';
  c := s[Length(s)];
  WriteLn(c, ' ', Ord(c));
  c := UpCase(s[2]);
  WriteLn(c)
end.
