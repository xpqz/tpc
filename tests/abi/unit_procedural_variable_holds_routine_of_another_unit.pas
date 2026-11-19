program UnitProceduralVariableHoldsRoutineOfAnotherUnit;
{$F+}
{ a unit-level procedural variable is assigned by the program and called by the unit }
uses Hooks;
procedure Mine(s: string);
begin
  WriteLn('mine: ', s);
end;
begin
  Fire('before');
  OnEvent := Mine;
  Fire('after');
  Fire('again');
  OnEvent := Ignore;
  Fire('last');
  WriteLn(Fired);
end.
