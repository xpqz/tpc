program ConsoleReadlnWithoutArgumentsSkipsALine;
var
  s: string;
begin
  ReadLn;
  ReadLn;
  ReadLn(s);
  WriteLn(s);
  ReadLn;
  WriteLn(Eof);
end.
