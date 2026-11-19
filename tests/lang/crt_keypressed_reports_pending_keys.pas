program KeyPending;
uses Crt;
var
  C: Char;
  N: Integer;
begin
  ClrScr;
  WriteLn(KeyPressed);
  WriteLn(KeyPressed);
  C := ReadKey;
  WriteLn(C, ' ', KeyPressed);
  C := ReadKey;
  WriteLn(C, ' ', KeyPressed);
  C := ReadKey;
  WriteLn(Ord(C), ' ', KeyPressed);
  C := ReadKey;
  WriteLn(Ord(C), ' ', KeyPressed);
  N := 0;
  while KeyPressed do
  begin
    C := ReadKey;
    Inc(N);
  end;
  WriteLn(N, ' ', KeyPressed);
end.
