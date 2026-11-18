program NestedRoutineShadowsEnclosingName;
{ a nested routine may reuse the name of its enclosing routine: inside the enclosing routine
  the name then means the nested one }
procedure Outer;
  procedure Outer;
  begin
    WriteLn('inner')
  end;
begin
  Outer;
  WriteLn('outer')
end;
begin
  Outer
end.
