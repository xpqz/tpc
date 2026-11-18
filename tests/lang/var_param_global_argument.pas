program VarParamGlobalArgument;
{ a global variable passed by reference to a routine that calls another routine }
var counter: Integer;
procedure Tick(var c: Integer);
begin
  c := c + 1
end;
procedure TickTwice(var c: Integer);
begin
  Tick(c);
  Tick(c)
end;
begin
  counter := 0;
  TickTwice(counter);
  Tick(counter);
  WriteLn(counter)
end.
