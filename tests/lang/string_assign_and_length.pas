program StringAssign;
var s: String; t: String[10]; u: string[1];
begin
  s := 'hello';
  WriteLn(s, ' ', Length(s));
  s := '';
  WriteLn('[', s, '] ', Length(s));
  t := 'abcdefghij';
  WriteLn(t, ' ', Length(t));
  u := 'x';
  WriteLn(u, ' ', Length(u));
  s := t; t := s;
  WriteLn(s, ' ', Length(s), ' ', SizeOf(s), ' ', SizeOf(t), ' ', SizeOf(u))
end.
