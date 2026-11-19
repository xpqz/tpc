program ReadLnEdit;
uses Crt;
var
  S: string;
begin
  ClrScr;
  ReadLn(S);
  WriteLn('[', S, ']');
  ReadLn(S);
  WriteLn('[', S, ']');
  ReadLn(S);
  WriteLn('[', S, ']');
  WriteLn(WhereX, ',', WhereY);
end.
