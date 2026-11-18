program NegTest;
{$F+}
type TProc = procedure;
var p: TProc;
procedure A;
begin
end;
begin
  p := @A
end.
