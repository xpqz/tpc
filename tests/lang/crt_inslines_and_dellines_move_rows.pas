program InsDel;
uses Crt;
var
  Row: Integer;
begin
  ClrScr;
  for Row := 1 to 25 do
  begin
    GotoXY(1, Row);
    if Row < 25 then
      Write('row ', Row)
    else
      Write('row 25');
  end;
  GotoXY(3, 5);
  InsLine;
  Write(WhereX, ',', WhereY);
  GotoXY(1, 10);
  DelLine;
  GotoXY(1, 25);
  DelLine;
  GotoXY(1, 1);
  TextAttr := $4F;
  InsLine;
  Write('new top');
  GotoXY(1, 12);
  TextAttr := $17;
  DelLine;
  GotoXY(1, 20);
  Write('end');
end.
