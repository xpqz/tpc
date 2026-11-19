program CornerScroll;
uses Crt;
begin
  ClrScr;
  GotoXY(1, 24);
  Write('above');
  GotoXY(1, 25);
  Write('bottom');
  GotoXY(80, 25);
  Write('X');
  Write(WhereX, ',', WhereY);
  GotoXY(80, 25);
  TextAttr := $4F;
  Write('Y');
  WriteLn(WhereX, ',', WhereY);
end.
