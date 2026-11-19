program ConsoleLineOfTwoHundredCharacters;
var
  s: string;
  n: Integer;
begin
  n := 0;
  while not Eof do
  begin
    ReadLn(s);
    n := n + 1;
    WriteLn(n, ' ', Length(s), ' ', s[1], s[Length(s)]);
  end;
end.
