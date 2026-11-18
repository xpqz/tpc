program NegTest;
{$F+}
type TProc = procedure(a, b: Integer);
var p: TProc;
procedure Two(a, b: Integer);
begin
end;
begin
  p := Two;
  p(1)
end.
