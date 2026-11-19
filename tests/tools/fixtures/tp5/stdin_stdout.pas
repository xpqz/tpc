program StdinStdout;
var s: string;
begin
  while not Eof do
  begin
    ReadLn(s);
    WriteLn('[', s, ']')
  end
end.
