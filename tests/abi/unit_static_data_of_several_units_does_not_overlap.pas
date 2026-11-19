program UnitStaticDataOfSeveralUnitsDoesNotOverlap;
{ globals, typed constants and strings of three units, each filled and read back }
uses Aa, Bb, Cc;
var
  here: array[1..5] of Integer;
  i: Integer;
begin
  for i := 1 to 5 do
    here[i] := 1000 + i;
  FillA;
  FillB;
  FillC;
  WriteLn(SumA, ' ', SumB, ' ', SumC);
  WriteLn(NameA, NameB, NameC);
  WriteLn(TagA, TagB, TagC);
  WriteLn(here[1], ' ', here[5]);
  WriteLn(LastA, ' ', LastB, ' ', LastC);
end.
