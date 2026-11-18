program NestedProcedureBasic;
{ a procedure declared inside a procedure and called from its body }
procedure Outer;
  procedure Inner;
  begin
    WriteLn('inner')
  end;
begin
  WriteLn('outer start');
  Inner;
  Inner;
  WriteLn('outer end')
end;
begin
  Outer
end.
