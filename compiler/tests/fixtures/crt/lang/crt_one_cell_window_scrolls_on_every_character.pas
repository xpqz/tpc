program OneCell;
uses Crt;
begin
  ClrScr;
  Write('before');
  Window(10, 5, 10, 5);
  Write('abc');
  Write(WhereX, ',', WhereY);
  Window(1, 1, 80, 25);
  GotoXY(1, 10);
  Write('after');
end.
