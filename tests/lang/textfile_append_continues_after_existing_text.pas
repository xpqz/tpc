program TextfileAppendContinuesAfterExistingText;
var
  f: Text;
  s: string;
begin
  Assign(f, 'LOG.TXT');
  Rewrite(f);
  WriteLn(f, 'one');
  Close(f);
  Append(f);
  WriteLn(f, 'two');
  Write(f, 'th');
  Write(f, 'ree');
  WriteLn(f);
  Close(f);
  Append(f);
  WriteLn(f, 'four');
  Close(f);
  Reset(f);
  while not Eof(f) do
  begin
    ReadLn(f, s);
    WriteLn('[', s, ']');
  end;
  Close(f);
end.
