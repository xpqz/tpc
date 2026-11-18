program StringConcatOverflow;
var s: String; t: String[6]; i: Integer;
begin
  s := '';
  for i := 1 to 30 do s := s + '0123456789';
  WriteLn(Length(s));
  WriteLn(Copy(s, 250, 10));
  t := 'abcd' + 'efgh';
  WriteLn(t, ' ', Length(t));
  t := 'abcd'; t := t + 'xyz' + 'w';
  WriteLn(t, ' ', Length(t));
  s := s + s;
  WriteLn(Length(s))
end.
