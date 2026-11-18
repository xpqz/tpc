program NestedGlobalRoutineCallFromNested;
{ a nested routine calls a global routine that was declared earlier }
procedure Shout(n: Integer);
begin
  WriteLn('<', n, '>')
end;
procedure Outer;
  procedure Inner(k: Integer);
  begin
    Shout(k * 2)
  end;
begin
  Inner(1);
  Inner(21)
end;
begin
  Outer
end.
