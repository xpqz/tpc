program NegTest;
{$F+}
type TProc = procedure(a: Integer; b: Boolean);
var p: TProc;
procedure Two(a: Integer; b: Boolean);
begin
end;
begin
  p := Two;
  p(1, 2)
end.
