program Names;
uses Crt;
var
  Delay: Integer;
procedure Beep;
begin
  Write('mine ');
end;
function WhereX: Integer;
begin
  WhereX := 42;
end;
begin
  Crt.ClrScr;
  Delay := 7;
  Crt.Delay(1);
  Beep;
  WriteLn(WhereX, ' ', Crt.WhereX, ' ', Delay);
  Crt.GotoXY(3, 4);
  Write(Crt.WhereY);
  GotoXY(1, 6);
  Write(Crt.KeyPressed);
end.
