program ReadLnOdd;
uses Crt;
var
  S: string;
begin
  ClrScr;
  ReadLn(S);
  WriteLn('[', S, ']', Length(S));
  ReadLn(S);
  WriteLn('[', S, ']', Length(S));
end.
