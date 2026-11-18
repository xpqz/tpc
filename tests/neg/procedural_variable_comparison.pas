program NegTest;
{$F+}
type TProc = procedure;
var p, q: TProc;
procedure A;
begin
end;
begin
  p := A;
  q := p;
  if p = q then WriteLn('same')
end.
