program DosNamesCanBeQualifiedAndVariablesReused;
uses Dos;
var
  a, b, c, d: Word;
begin
  Dos.GetDate(a, b, c, d);
  WriteLn((b >= 1) and (b <= 12));
  Dos.GetTime(a, b, c, d);
  WriteLn(a <= 23);
  GetDate(a, b, c, d);
  WriteLn(d <= 6);
end.
