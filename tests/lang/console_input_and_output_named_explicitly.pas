program ConsoleInputAndOutputNamedExplicitly;
var
  a, b: Integer;
  s: string;
begin
  ReadLn(Input, a, b);
  WriteLn(Output, a + b);
  ReadLn(Input, s);
  Write(Output, 'got ');
  WriteLn(Output, s);
  WriteLn(Eof(Input), ' ', Eoln(Input));
  ReadLn(Input);
  WriteLn(Eof(Input));
end.
