program StringTruncate;
var a: String[3]; b: String[5]; c: String[1]; s: String;
begin
  a := 'abcdef';
  WriteLn(a, ' ', Length(a));
  s := 'hello world';
  b := s;
  WriteLn(b, ' ', Length(b));
  c := s;
  WriteLn(c, ' ', Length(c));
  a := b;
  WriteLn(a, ' ', Length(a));
  c := 'xyz';
  WriteLn(c);
  a := b + c;
  WriteLn(a, ' ', Length(a));
  b := 'ab'; a := b;
  WriteLn(a, ' ', Length(a))
end.
