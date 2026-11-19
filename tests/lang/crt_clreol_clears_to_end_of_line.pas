program ClrEolBasic;
uses Crt;
var
  Row: Integer;
begin
  ClrScr;
  for Row := 1 to 4 do
  begin
    GotoXY(1, Row);
    Write('0123456789 0123456789 0123456789');
  end;
  GotoXY(12, 2);
  ClrEol;
  Write(WhereX, ',', WhereY);
  GotoXY(1, 3);
  ClrEol;
  GotoXY(34, 4);
  ClrEol;
  GotoXY(80, 5);
  ClrEol;
  TextAttr := $2E;
  GotoXY(5, 4);
  ClrEol;
  GotoXY(1, 10);
  Write('end');
end.
