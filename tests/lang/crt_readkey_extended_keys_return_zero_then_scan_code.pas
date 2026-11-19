program ReadKeyExtended;
uses Crt;
var
  I: Integer;
  C: Char;
begin
  ClrScr;
  for I := 1 to 10 do
  begin
    C := ReadKey;
    if C = #0 then
    begin
      C := ReadKey;
      WriteLn('extended ', Ord(C));
    end
    else
      WriteLn('plain ', Ord(C));
  end;
end.
