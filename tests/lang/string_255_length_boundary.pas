program String255;
var s: String; t: String[255]; i: Integer;
begin
  s := '';
  for i := 1 to 255 do s := s + Chr(65 + i mod 26);
  WriteLn(Length(s), ' ', Ord(s[0]));
  t := s;
  WriteLn(Length(t), ' ', t = s, ' ', SizeOf(s), ' ', SizeOf(t));
  s := s + 'x';
  WriteLn(Length(s), ' ', s = t);
  WriteLn(s[255], s[1]);
  Delete(s, 1, 250);
  WriteLn(s)
end.
