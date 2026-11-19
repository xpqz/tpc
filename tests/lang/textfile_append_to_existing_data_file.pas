program TextfileAppendToExistingDataFile;
var
  f: Text;
  s: string;
begin
  Assign(f, 'NOTES.TXT');
  Append(f);
  WriteLn(f, 'added by program');
  Close(f);
  Reset(f);
  while not Eof(f) do
  begin
    ReadLn(f, s);
    WriteLn(s);
  end;
  Close(f);
end.
