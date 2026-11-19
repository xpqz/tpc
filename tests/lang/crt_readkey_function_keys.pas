program ReadKeyFunction;
uses Crt;
var
  I: Integer;
  C: Char;
begin
  ClrScr;
  for I := 1 to 10 do
  begin
    C := ReadKey;
    Write(Ord(C), ':');
    C := ReadKey;
    Write(Ord(C), ' ');
  end;
  WriteLn;
end.
