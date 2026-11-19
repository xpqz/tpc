program DosGetDateValuesAreInRange;
{ the date itself is never printed, only that it is plausible }
uses Dos;
var
  year, month, day, weekday: Word;
begin
  GetDate(year, month, day, weekday);
  WriteLn(year >= 1980);
  WriteLn(year <= 2099);
  WriteLn((month >= 1) and (month <= 12));
  WriteLn((day >= 1) and (day <= 31));
  WriteLn(weekday <= 6);
end.
