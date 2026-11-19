program WindowBasic;
uses Crt;
begin
  ClrScr;
  Write('outside');
  Window(10, 5, 30, 10);
  WriteLn(WhereX, ',', WhereY);
  Write('inside');
  WriteLn(WhereX, ',', WhereY);
  GotoXY(1, 1);
  Write('top');
  GotoXY(21, 6);
  Write('Z');
  GotoXY(5, 3);
  Write('mid');
  Window(1, 1, 80, 25);
  GotoXY(1, 20);
  Write('full ', WhereX, ',', WhereY);
end.
