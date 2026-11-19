program ConsoleEofOnEmptyInput;
var
  s: string;
  n: Integer;
begin
  WriteLn(Eof);
  ReadLn(s);
  WriteLn('[', s, ']');
  Read(n);
  WriteLn(n);
end.
