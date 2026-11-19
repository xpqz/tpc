program ConsoleEolnReportsEndOfLine;
var
  c: Char;
  letters, lines: Integer;
begin
  lines := 0;
  while not Eof do
  begin
    letters := 0;
    while not Eoln do
    begin
      Read(c);
      letters := letters + 1;
    end;
    ReadLn;
    lines := lines + 1;
    WriteLn('line ', lines, ': ', letters, ' characters');
  end;
end.
