program IncludeRoutinesBetweenDeclarations;
var
  total: Integer;
{$I ROUTINES.INC}
begin
  total := 0;
  Add(5);
  Add(7);
  WriteLn(total);
  WriteLn(Double(total));
end.
