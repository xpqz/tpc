program MixedInput;
uses Crt;
var
  C: Char;
  S: string;
  N: Integer;
begin
  ClrScr;
  C := ReadKey;
  ReadLn(S);
  WriteLn(C, '/', S);
  if KeyPressed then
    WriteLn('key waiting');
  ReadLn(N);
  C := ReadKey;
  WriteLn(N, '/', C);
end.
