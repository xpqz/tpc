program ReadLnShort;
uses Crt;
var
  S: string[4];
  T: string;
begin
  ClrScr;
  ReadLn(S);
  WriteLn('[', S, ']');
  ReadLn(T);
  WriteLn('[', T, ']');
end.
