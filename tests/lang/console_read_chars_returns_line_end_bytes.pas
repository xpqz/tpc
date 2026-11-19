program ConsoleReadCharsReturnsLineEndBytes;
var
  c: Char;
begin
  while not Eof do
  begin
    Read(c);
    if c = #13 then Write('<CR>')
    else if c = #10 then Write('<LF>')
    else Write(c);
  end;
  WriteLn;
end.
