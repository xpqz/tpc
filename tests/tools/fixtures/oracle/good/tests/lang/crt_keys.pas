program Keys;
uses Dos,
  Crt;
var
  C: Char;
  S: string;
begin
  ClrScr;
  WriteLn(KeyPressed);
  C := ReadKey;
  WriteLn(Ord(C));
  C := ReadKey;
  WriteLn(Ord(C));
  C := ReadKey;
  WriteLn(Ord(C), ' ', Ord(C));
  C := ReadKey;
  WriteLn(Ord(C));
  C := ReadKey;
  WriteLn(Ord(C));
  ReadLn(S);
  WriteLn('[', S, ']');
  while KeyPressed do C := ReadKey;
  WriteLn(KeyPressed);
  TextAttr := $1E;
  GotoXY(10, 12);
  Write('colour');
end.
