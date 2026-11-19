program WindowClrEol;
uses Crt;
var
  Row: Integer;
begin
  ClrScr;
  for Row := 1 to 6 do
  begin
    GotoXY(1, Row);
    Write('abcdefghijklmnopqrstuvwxyz0123456789');
  end;
  Window(5, 2, 12, 5);
  GotoXY(3, 2);
  ClrEol;
  Write(WhereX, ',', WhereY);
  GotoXY(1, 3);
  TextBackground(Red);
  ClrEol;
  Window(1, 1, 80, 25);
  GotoXY(1, 10);
  Write('after');
end.
