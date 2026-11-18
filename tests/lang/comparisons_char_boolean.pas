program ComparisonsCharBoolean;
var
  c, d: Char;
  a, b: Boolean;
begin
  c := 'a';
  d := 'b';
  WriteLn(c < d, ' ', c > d, ' ', c = 'a', ' ', c <> d, ' ', 'A' < 'a', ' ', '0' < '9');
  a := False;
  b := True;
  WriteLn(a < b, ' ', a > b, ' ', a = b, ' ', a <> b, ' ', a <= a, ' ', b >= a)
end.
