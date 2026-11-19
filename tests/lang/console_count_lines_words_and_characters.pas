program ConsoleCountLinesWordsAndCharacters;
var
  s: string;
  lines, words, chars, i: Integer;
  inWord: Boolean;
begin
  lines := 0;
  words := 0;
  chars := 0;
  while not Eof do
  begin
    ReadLn(s);
    lines := lines + 1;
    chars := chars + Length(s);
    inWord := False;
    for i := 1 to Length(s) do
      if s[i] = ' ' then
        inWord := False
      else
      begin
        if not inWord then words := words + 1;
        inWord := True;
      end;
  end;
  WriteLn(lines, ' ', words, ' ', chars);
end.
