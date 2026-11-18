program NegTest;
{$F+}
type TProc = procedure;
var p: TProc;
function F: Integer;
begin
  F := 1
end;
begin
  p := F
end.
