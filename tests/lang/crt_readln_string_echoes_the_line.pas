program ReadLnString;
uses Crt;
var
  S: string;
begin
  ClrScr;
  Write('name? ');
  ReadLn(S);
  WriteLn('hello ', S, '!');
  Write('again? ');
  ReadLn(S);
  WriteLn('[', S, '] ', Length(S));
  WriteLn(WhereX, ',', WhereY);
end.
