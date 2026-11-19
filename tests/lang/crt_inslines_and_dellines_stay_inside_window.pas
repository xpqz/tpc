program InsDelWindow;
uses Crt;
var
  Row: Integer;
begin
  ClrScr;
  for Row := 1 to 10 do
  begin
    GotoXY(1, Row);
    Write('0123456789ABCDEFGHIJ', Row mod 10);
  end;
  Window(5, 3, 14, 7);
  GotoXY(1, 2);
  InsLine;
  GotoXY(1, 4);
  DelLine;
  TextAttr := $2F;
  GotoXY(1, 5);
  InsLine;
  GotoXY(1, 1);
  DelLine;
  Window(1, 1, 80, 25);
  GotoXY(1, 15);
  Write('end');
end.
