program CharPlusChar;
var a, b: Char; s: String;
begin
  a := 'A'; b := 'B';
  WriteLn(a + b);
  s := a + b;
  WriteLn(Length(s), ' ', Length(a + b), ' ', Length(a + b + 'CD'));
  WriteLn(a + b = 'AB', ' ', a + b < 'AC')
end.
