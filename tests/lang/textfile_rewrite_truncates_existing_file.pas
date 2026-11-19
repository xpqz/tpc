program TextfileRewriteTruncatesExistingFile;
var
  f: Text;
  s: string;
  lines: Integer;
begin
  Assign(f, 'OLD.TXT');
  Reset(f);
  lines := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    lines := lines + 1;
  end;
  Close(f);
  WriteLn('before: ', lines);
  Rewrite(f);
  WriteLn(f, 'new');
  Close(f);
  Reset(f);
  lines := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    WriteLn(s);
    lines := lines + 1;
  end;
  Close(f);
  WriteLn('after: ', lines);
end.
