program ProcVarChosenByCondition;
{$F+}
{ a dispatch loop: the variable is chosen each iteration, the callee changes a global }
type TAct = procedure;
var state: Integer; act: TAct; step: Integer;
procedure Up; begin state := state + 3 end;
procedure Down; begin state := state - 1 end;
procedure Reset; begin state := 0 end;
begin
  state := 0;
  for step := 1 to 12 do begin
    if step mod 5 = 0 then act := Reset
    else if Odd(step) then act := Up
    else act := Down;
    act;
    Write(state, ' ')
  end;
  WriteLn
end.
