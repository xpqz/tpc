program ReadLnWait;
uses Crt;
begin
  ClrScr;
  Write('press enter');
  ReadLn;
  WriteLn('went on ', WhereX, ',', WhereY);
end.
