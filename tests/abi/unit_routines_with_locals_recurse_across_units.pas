program UnitRoutinesWithLocalsRecurseAcrossUnits;
uses Even, Odd;
begin
  WriteLn(IsEven(10), ' ', IsEven(7));
  WriteLn(IsOdd(10), ' ', IsOdd(7));
  WriteLn(Depth);
end.
