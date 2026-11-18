program ExitMainProgram;
{ Exit in the main program ends the program }
procedure P;
begin
  WriteLn('in p')
end;
var i: Integer;
begin
  P;
  for i := 1 to 5 do begin
    WriteLn(i);
    if i = 3 then Exit
  end;
  WriteLn('not reached')
end.
