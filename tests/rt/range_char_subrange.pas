program RangeChar;
{$R+}
var c: Char; l: 'a'..'f';
begin
  WriteLn('before');
  c := 'z';
  l := c;
  WriteLn(l);
  WriteLn('not reached')
end.
