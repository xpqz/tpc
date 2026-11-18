program StringPlus;
var a, b, c: String;
begin
  a := 'foo'; b := 'bar';
  c := a + b;
  WriteLn(c, ' ', Length(c));
  c := a + '-' + b + '!';
  WriteLn(c);
  c := '' + a;
  WriteLn(c);
  c := a + '' + '';
  WriteLn(c, Length(c));
  c := a + a + a;
  a := a + 'x';
  WriteLn(c, ' ', a);
  a := a + a;
  WriteLn(a);
  WriteLn(a + b, ' ', Length(a + b))
end.
