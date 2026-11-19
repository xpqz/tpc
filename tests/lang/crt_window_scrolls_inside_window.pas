program WindowScroll;
uses Crt;
var
  I: Integer;
begin
  ClrScr;
  GotoXY(1, 1);
  WriteLn('left margin kept');
  GotoXY(1, 24);
  Write('bottom kept');
  Window(20, 5, 39, 9);
  for I := 1 to 9 do
    WriteLn('line ', I);
  Write('end');
  Write(' ', WhereX, ',', WhereY);
  Window(50, 2, 60, 4);
  TextAttr := $1F;
  for I := 1 to 6 do
    Write(I, ' ');
  Window(1, 1, 80, 25);
  GotoXY(1, 15);
  Write(WhereX, ',', WhereY);
end.
