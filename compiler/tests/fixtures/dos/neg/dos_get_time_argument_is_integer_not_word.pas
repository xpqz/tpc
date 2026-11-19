program DosGetTimeArgumentIsIntegerNotWord;
uses Dos;
var
  h, m, s, c: Word;
  bad: Integer;
begin
  GetTime(h, m, s, bad);
end.
