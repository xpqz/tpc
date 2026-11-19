program DosGetTimeValuesAreInRange;
{ the time itself is never printed, only that it is plausible }
uses Dos;
var
  hour, minute, second, hundredths: Word;
begin
  GetTime(hour, minute, second, hundredths);
  WriteLn(hour <= 23);
  WriteLn(minute <= 59);
  WriteLn(second <= 59);
  WriteLn(hundredths <= 99);
end.
