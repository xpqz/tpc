program GotoRange;
uses Crt;
begin
  ClrScr;
  GotoXY(10, 5);
  GotoXY(0, 5);
  Write(WhereX, ',', WhereY, ' ');
  GotoXY(10, 5);
  GotoXY(81, 5);
  Write(WhereX, ',', WhereY, ' ');
  GotoXY(10, 5);
  GotoXY(10, 0);
  Write(WhereX, ',', WhereY, ' ');
  GotoXY(10, 7);
  GotoXY(10, 26);
  Write(WhereX, ',', WhereY, ' ');
  GotoXY(10, 9);
  GotoXY(255, 255);
  Write(WhereX, ',', WhereY, ' ');
  GotoXY(80, 25);
  Write(WhereX, ',', WhereY);
end.
