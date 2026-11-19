program UnitTextFileVariableOwnedByUnit;
uses Journal;
var
  i: Integer;
begin
  OpenJournal;
  for i := 1 to 3 do
    Note(i);
  CloseJournal;
  Replay;
end.
