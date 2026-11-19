program TypedconstStrings;
const
  s: string = 'hello';
  t: string[3] = 'abc';
  e: string = '';
  q: string[10] = 'it''s';
  ch: string[1] = 'x';
begin
  WriteLn(s, ' ', Length(s));
  WriteLn(t, ' ', Length(t));
  WriteLn('[', e, ']', Length(e));
  WriteLn(q);
  WriteLn(ch);
  s := s + ' world';
  WriteLn(s);
  t := 'wxyz';
  WriteLn(t);
  e := 'filled';
  WriteLn(e);
  s[1] := 'J';
  WriteLn(s);
end.
