program NoScript;
uses Crt;
begin
  ClrScr;
  WriteLn(KeyPressed);
  if not KeyPressed then
    WriteLn('idle');
end.
