program UnitInitializationOrderFollowsUsesList;
{ independent units initialize in the order of the uses list }
uses Second, First;
begin
  WriteLn('program');
end.
