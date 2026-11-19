program DataFile;
var f: text; s: string;
begin
  Assign(f, 'DATA.TXT');
  Reset(f);
  while not Eof(f) do
  begin
    ReadLn(f, s);
    WriteLn(s)
  end;
  Close(f)
end.
