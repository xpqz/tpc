program ScrolledWindow;
uses Crt;
var i: Integer;
begin
  ClrScr;
  Window(20, 5, 30, 8);
  for i := 1 to 7 do
    WriteLn('row', i);
  Write(WhereX, ' ', WhereY, ' ');
  GotoXY(11, 4);
  Write(WhereX, ' ', WhereY);
  GotoXY(12, 4);
  GotoXY(1, 5);
  Write('!');
end.
