program UnitTypesEnumerationsAndSetsExported;
uses Days;
var
  d: TDay;
  off: TDaySet;
begin
  off := Weekend;
  for d := Mon to Sun do
    if d in off then
      WriteLn(DayName(d), ' off')
    else
      WriteLn(DayName(d), ' work');
  WriteLn(Ord(Succ(Fri)));
  d := NextDay(Sun);
  WriteLn(DayName(d));
  off := off + [Fri];
  WriteLn(Fri in off, ' ', Thu in off);
end.
