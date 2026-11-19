program WrapRight;
uses Crt;
var
  I: Integer;
begin
  ClrScr;
  for I := 1 to 80 do
    Write(Chr(48 + I mod 10));
  Write('+');
  WriteLn;
  Write('0123456789012345678901234567890123456789');
  Write('0123456789012345678901234567890123456789');
  WriteLn;
  Write('0123456789012345678901234567890123456789');
  Write('0123456789012345678901234567890123456789');
  Write('wrapped');
  WriteLn(' ', WhereX, ',', WhereY);
  GotoXY(79, 10);
  Write('abcd');
  TextAttr := $5E;
  Write(WhereX, ',', WhereY);
end.
