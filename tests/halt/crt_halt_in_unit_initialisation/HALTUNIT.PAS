unit HaltUnit;
interface
procedure Hello;
implementation
procedure Hello;
begin
  WriteLn('hello')
end;
begin
  WriteLn('unit init');
  Halt(9);
  WriteLn('not reached')
end.
