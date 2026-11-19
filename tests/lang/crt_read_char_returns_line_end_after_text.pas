program ReadCharCrt;
uses Crt;
var
  C: Char;
begin
  ClrScr;
  Read(C);
  WriteLn('<', Ord(C), '>');
  Read(C);
  WriteLn('<', Ord(C), '>');
  ReadLn;
  WriteLn('end');
end.
