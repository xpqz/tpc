program TextfileWriteThenReadBack;
var
  f: Text;
  s: string;
  n: Integer;
begin
  Assign(f, 'OUT.TXT');
  Rewrite(f);
  WriteLn(f, 'first line');
  WriteLn(f, 'second line');
  WriteLn(f, 42);
  Close(f);
  Assign(f, 'OUT.TXT');
  Reset(f);
  n := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    n := n + 1;
    WriteLn(n, ': ', s);
  end;
  Close(f);
end.
