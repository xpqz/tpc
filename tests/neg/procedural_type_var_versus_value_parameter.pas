program NegTest;
{$F+}
type TProc = procedure(var x: Integer);
var p: TProc;
procedure ByValue(x: Integer);
begin
end;
begin
  p := ByValue
end.
