program WindowWrap;
uses Crt;
begin
  ClrScr;
  Window(20, 3, 29, 8);
  Write('0123456789ABCDEFGHIJ');
  Write('<');
  WriteLn(WhereX, ',', WhereY);
  Write('a');
  GotoXY(10, 5);
  Write('XY');
  Window(1, 1, 80, 25);
  GotoXY(1, 12);
  Write(WhereX, ',', WhereY);
end.
