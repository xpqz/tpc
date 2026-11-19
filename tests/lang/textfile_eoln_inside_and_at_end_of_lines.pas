program TextfileEolnInsideAndAtEndOfLines;
var
  f: Text;
  c: Char;
  words, letters: Integer;
begin
  Assign(f, 'WORDS.TXT');
  Reset(f);
  words := 0;
  while not Eof(f) do
  begin
    letters := 0;
    while not Eoln(f) do
    begin
      Read(f, c);
      letters := letters + 1;
    end;
    ReadLn(f);
    words := words + 1;
    WriteLn('line ', words, ' has ', letters);
  end;
  Close(f);
end.
