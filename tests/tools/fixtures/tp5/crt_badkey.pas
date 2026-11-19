program Exhausted;
uses Crt;
var
  C: Char;
begin
  C := ReadKey;
  Write(C);
  C := ReadKey;
  Write('unreachable');
end.
