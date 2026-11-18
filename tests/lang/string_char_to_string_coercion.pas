program CharToString;
var s: String; c, d: Char; t: String[3];
begin
  c := 'a'; d := 'b';
  s := c;
  WriteLn(s, Length(s));
  s := c + d;
  WriteLn(s, Length(s));
  s := c + d + c + 'z' + #65;
  WriteLn(s);
  t := c;
  WriteLn(t);
  s := d + s;
  WriteLn(s);
  s := '';
  s := s + c;
  WriteLn(s);
  s := #0;
  WriteLn(Length(s), Ord(s[1]))
end.
