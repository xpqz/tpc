program WriteWithoutArgumentsToAFile;
var
  f: Text;
  s: string;
  n: Integer;
begin
  Assign(f, 'EMPTY.TXT');
  Rewrite(f);
  Write(f);
  WriteLn(f);
  WriteLn(f);
  Write(f, 'x');
  WriteLn(f);
  Close(f);
  Assign(f, 'EMPTY.TXT');
  Reset(f);
  n := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    n := n + 1;
    WriteLn(n, ':', Length(s), ':', s);
  end;
  Close(f);
end.
