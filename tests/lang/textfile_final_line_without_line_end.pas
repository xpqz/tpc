program TextfileFinalLineWithoutLineEnd;
var
  f: Text;
  s: string;
begin
  Assign(f, 'NOEND.TXT');
  Reset(f);
  while not Eof(f) do
  begin
    ReadLn(f, s);
    WriteLn('[', s, ']');
  end;
  Close(f);
end.
