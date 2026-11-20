uses Crt;
var c: Char;
begin
  ClrScr;
  Write('waiting');
  repeat until KeyPressed;
  c := ReadKey;
  if c = #0 then c := ReadKey;
  GotoXY(1, 2);
  Write('got ', Ord(c));
end.
