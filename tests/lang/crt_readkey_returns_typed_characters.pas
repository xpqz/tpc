program ReadKeyChars;
uses Crt;
var
  I: Integer;
  C: Char;
begin
  ClrScr;
  for I := 1 to 9 do
  begin
    C := ReadKey;
    Write(Ord(C), ' ');
  end;
  WriteLn;
  WriteLn(WhereX, ',', WhereY);
end.
