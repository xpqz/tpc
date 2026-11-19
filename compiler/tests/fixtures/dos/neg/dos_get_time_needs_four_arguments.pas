program DosGetTimeNeedsFourArguments;
uses Dos;
var
  h, m, s: Word;
begin
  GetTime(h, m, s);
end.
