program ExitAtStartAndEnd;
{ Exit as the first and as the last statement; Exit under case and in a compound }
procedure Early(n: Integer);
begin
  Exit;
  WriteLn('never ', n)
end;
procedure Late(n: Integer);
begin
  case n of
    1: begin WriteLn('one'); Exit end;
    2: WriteLn('two')
  else
    begin WriteLn('other'); Exit end
  end;
  WriteLn('tail ', n);
  Exit
end;
function F: Integer;
begin
  F := 7;
  Exit
end;
begin
  Early(1);
  Late(1); Late(2); Late(3);
  WriteLn(F)
end.
