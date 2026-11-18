program NegTest;
{$F+}
type TProc = procedure;
procedure Call(p: TProc);
begin
  p
end;
procedure Outer;
  procedure Inner;
  begin
    WriteLn('inner')
  end;
begin
  Call(Inner)
end;
begin
  Outer
end.
