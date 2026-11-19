program DelaySound;
uses Crt;
var
  I: Integer;
begin
  ClrScr;
  Write('before');
  Sound(440);
  Delay(20);
  NoSound;
  Delay(0);
  Delay(1);
  for I := 1 to 3 do
  begin
    Sound(100 * I);
    Delay(10);
  end;
  NoSound;
  NoSound;
  WriteLn(' after ', WhereX, ',', WhereY);
  Sound(0);
  Sound(65535);
  NoSound;
  Write('done');
end.
