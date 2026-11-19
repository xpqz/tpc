program WindowHome;
uses Crt;
begin
  ClrScr;
  GotoXY(50, 20);
  Window(5, 5, 15, 8);
  Write(WhereX, ',', WhereY);
  GotoXY(4, 3);
  Window(30, 10, 40, 14);
  Write(WhereX, ',', WhereY);
  Window(1, 1, 80, 25);
  Write('[', WhereX, ',', WhereY, ']');
  GotoXY(1, 24);
  Write('x');
end.
