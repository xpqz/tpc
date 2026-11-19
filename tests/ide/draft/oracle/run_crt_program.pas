program A;
uses Crt;
var C: Char;
begin
  WriteLn('before');
  C := ReadKey;
  Write('got ', Ord(C));
end.
