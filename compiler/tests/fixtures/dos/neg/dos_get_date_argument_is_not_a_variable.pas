program DosGetDateArgumentIsNotAVariable;
uses Dos;
var
  y, m, d: Word;
begin
  GetDate(y, m, d, 3);
end.
