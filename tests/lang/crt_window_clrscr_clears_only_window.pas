program WindowClear;
uses Crt;
var
  Row: Integer;
begin
  ClrScr;
  for Row := 1 to 12 do
  begin
    GotoXY(1, Row);
    Write('................................................');
  end;
  TextBackground(Magenta);
  Window(10, 4, 20, 8);
  ClrScr;
  Write(WhereX, ',', WhereY, ' ');
  Write('in');
  Window(1, 1, 80, 25);
  GotoXY(1, 14);
  Write('done');
end.
