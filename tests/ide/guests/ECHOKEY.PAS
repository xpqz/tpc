program EchoKey;
uses Crt;
var
  C: Char;
begin
  C := ReadKey;
  Write('guest got ', Ord(C));
end.
