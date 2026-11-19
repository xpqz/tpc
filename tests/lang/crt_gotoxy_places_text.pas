program GotoPlaces;
uses Crt;
begin
  ClrScr;
  GotoXY(1, 1);
  Write('A');
  GotoXY(80, 1);
  Write('B');
  GotoXY(1, 25);
  Write('C');
  GotoXY(40, 13);
  Write('centre');
  GotoXY(5, 3);
  Write('x');
  GotoXY(5, 3);
  Write('y');
  GotoXY(10, 20);
  WriteLn(WhereX, ' ', WhereY);
  GotoXY(2, 24);
  Write('z');
end.
