program ScrollBottom;
uses Crt;
var
  I: Integer;
begin
  ClrScr;
  for I := 1 to 24 do
    WriteLn('line ', I);
  Write('line 25');
  Write(' ', WhereX, ',', WhereY);
  WriteLn;
  Write(WhereX, ',', WhereY);
  TextAttr := $1E;
  WriteLn;
  WriteLn('blue');
  TextAttr := $07;
  Write('last');
end.
