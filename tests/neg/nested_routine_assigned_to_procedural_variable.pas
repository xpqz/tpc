program NegTest;
{$F+}
type TProc = procedure;
var p: TProc;
procedure Outer;
  procedure Inner;
  begin
    WriteLn('inner')
  end;
begin
  p := Inner
end;
begin
  Outer;
  p
end.
