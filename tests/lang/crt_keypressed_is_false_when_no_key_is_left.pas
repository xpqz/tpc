program NoKeys;
uses Crt;
var
  N: Integer;
begin
  ClrScr;
  N := 0;
  while KeyPressed do
  begin
    if ReadKey = #0 then;
    Inc(N);
  end;
  WriteLn(N, ' ', KeyPressed);
  WriteLn(KeyPressed);
end.
