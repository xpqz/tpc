program UnitUnitVariablesPassedByReferenceToOtherUnit;
uses Data, Mutate;
begin
  Init;
  Double(Counter);
  Double(Counter);
  WriteLn(Counter);
  Rename(Title);
  WriteLn(Title);
  Rotate(Point);
  WriteLn(Point.x, ' ', Point.y);
  Fill(Cells);
  WriteLn(Cells[1], Cells[2], Cells[3]);
end.
