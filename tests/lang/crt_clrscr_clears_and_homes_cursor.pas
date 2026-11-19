program ClrScrHomes;
uses Crt;
var
  Row: Integer;
begin
  for Row := 1 to 30 do
    WriteLn('row ', Row);
  Write('tail');
  ClrScr;
  WriteLn(WhereX, ',', WhereY);
  Write('after');
  WriteLn(' ', WhereX, ',', WhereY);
end.
