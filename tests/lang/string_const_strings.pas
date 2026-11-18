program StringConst;
const Greeting = 'Hello'; Sep = ', '; Both = Greeting + Sep; Q = '''';
var s: String;
begin
  s := Greeting + Sep + 'World';
  WriteLn(s, ' ', Length(s));
  WriteLn(Both, '|', Length(Both), '|', Q, Length(Q));
  WriteLn(Length(Greeting))
end.
