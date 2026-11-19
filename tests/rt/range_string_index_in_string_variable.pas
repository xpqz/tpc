program RangeString;
{$R+}
var s: String[3]; c: Char; i: Integer;
begin
  s := 'abc';
  i := 4;
  c := s[i];
  WriteLn('read ok');
  i := 0;
  c := s[i];
  WriteLn(Ord(c));
  c := 'x';
  WriteLn(s[3], s[1])
end.
