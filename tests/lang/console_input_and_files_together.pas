program ConsoleInputAndFilesTogether;
var
  f: Text;
  name, s: string;
begin
  ReadLn(name);
  Assign(f, 'GREET.TXT');
  Rewrite(f);
  WriteLn(f, 'Hello, ', name);
  Close(f);
  Reset(f);
  ReadLn(f, s);
  WriteLn(s);
  Close(f);
  ReadLn(name);
  WriteLn('second: ', name);
end.
