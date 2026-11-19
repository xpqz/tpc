program ReportLineStatisticsOfATextFile;
var
  f, out: Text;
  line: string;
  lines, longest, longestAt, words, i: Integer;
  inWord: Boolean;
begin
  Assign(f, 'POEM.TXT');
  Reset(f);
  lines := 0;
  longest := 0;
  longestAt := 0;
  words := 0;
  while not Eof(f) do
  begin
    ReadLn(f, line);
    lines := lines + 1;
    if Length(line) > longest then
    begin
      longest := Length(line);
      longestAt := lines;
    end;
    inWord := False;
    for i := 1 to Length(line) do
      if line[i] <> ' ' then
      begin
        if not inWord then words := words + 1;
        inWord := True;
      end
      else
        inWord := False;
  end;
  Close(f);
  Assign(out, 'STATS.TXT');
  Rewrite(out);
  WriteLn(out, 'lines=', lines);
  WriteLn(out, 'words=', words);
  WriteLn(out, 'longest=', longest, ' at line ', longestAt);
  Close(out);
  Reset(out);
  while not Eof(out) do
  begin
    ReadLn(out, line);
    WriteLn(line);
  end;
  Close(out);
end.
