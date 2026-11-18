program RangeStr;
{$R+}
var s: String[3]; c: Char; i: Integer;
begin
  WriteLn('before');
  i := 4;
  c := s[i];
  WriteLn(c);
  WriteLn('not reached')
end.
