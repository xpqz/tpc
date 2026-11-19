program ConsoleEchoLinesUntilEndOfInput;
var
  s: string;
  n: Integer;
begin
  n := 0;
  while not Eof do
  begin
    ReadLn(s);
    n := n + 1;
    WriteLn(n, ': ', s);
  end;
  WriteLn('lines: ', n);
end.
