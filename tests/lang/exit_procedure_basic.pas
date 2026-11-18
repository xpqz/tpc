program ExitProcedureBasic;
{ Exit leaves the procedure at once }
procedure Check(n: Integer);
begin
  if n < 0 then begin
    WriteLn('negative');
    Exit
  end;
  WriteLn('ok ', n)
end;
begin
  Check(5);
  Check(-5);
  Check(0)
end.
