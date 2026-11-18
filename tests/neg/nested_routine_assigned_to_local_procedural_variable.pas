program NegTest;
{$F+}
type TProc = procedure;
procedure Outer;
var q: TProc;
  procedure Inner;
  begin
  end;
begin
  q := Inner;
  q
end;
begin
  Outer
end.
