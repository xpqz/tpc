program StringAliasingAndSelfAssignment;
var s, t: String; u: String[6];
begin
  s := 'abc';
  s := s + s;
  WriteLn(s);
  Insert(s, s, 2);
  WriteLn(s);
  s := Copy(s, 2, 3) + s;
  WriteLn(s);
  s := 'xyz';
  s := s[3] + s[2] + s[1] + s;
  WriteLn(s);
  Delete(s, 1, 3);
  WriteLn(s);
  s := s;
  t := s + s;
  s := t + s;
  WriteLn(s, ' ', Length(s));
  u := 'abcd';
  u := u + u;
  WriteLn(u, ' ', Length(u));
  Insert(u, u, 1);
  WriteLn(u);
  s := 'ab';
  s := s + s + s + s;
  s := s + s + s + s;
  WriteLn(s, Length(s))
end.
