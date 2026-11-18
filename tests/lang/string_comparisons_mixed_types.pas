program StringCompareMixed;
var a: String[3]; b: String[20]; c: String;
begin
  a := 'abc'; b := 'abc'; c := 'abd';
  WriteLn(a = b, ' ', a < c, ' ', b > c, ' ', a = 'abc', ' ', 'abc' = b, ' ', c > 'abc');
  b := 'abcdef'; a := b;
  WriteLn(a = b, ' ', a < b, ' ', a = 'abc');
  c := a + 'd';
  WriteLn(c = 'abcd', ' ', Length(c))
end.
