program NoEcho;
uses Crt;
var
  C: Char;
begin
  ClrScr;
  Write('press: ');
  C := ReadKey;
  Write('[', C, ']');
  C := ReadKey;
  C := ReadKey;
  WriteLn(WhereX, ',', WhereY);
end.
