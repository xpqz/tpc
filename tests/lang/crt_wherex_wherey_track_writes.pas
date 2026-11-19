program WherePos;
uses Crt;
procedure Show;
begin
  WriteLn('[', WhereX, ' ', WhereY, ']');
end;
begin
  ClrScr;
  Show;
  Write('abc');
  Show;
  Write('');
  Write(#13);
  Show;
  WriteLn;
  Show;
  Write('0123456789012345678901234567890123456789012345678901234567890123456789012345678');
  Show;
  Write('x');
  Show;
  Write('y');
  Show;
  Write('z');
  Show;
end.
