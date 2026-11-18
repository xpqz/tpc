program ProcTypeBasic;
{$F+}
{ procedural type, variable assignment and call }
type TProc = procedure;
var p: TProc;
procedure Hi;
begin
  WriteLn('hi')
end;
procedure Bye;
begin
  WriteLn('bye')
end;
begin
  p := Hi;
  p;
  p := Bye;
  p
end.
