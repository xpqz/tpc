program ThousandsOfLinesAreWrittenAndCounted;
var
  f: Text;
  i, n, total, v: Integer;
  s: string;
begin
  Assign(f, 'BIG.TXT');
  Rewrite(f);
  for i := 1 to 3000 do
    WriteLn(f, 'Line number ', i, ' of the generated file');
  Close(f);
  Reset(f);
  n := 0;
  total := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    n := n + 1;
    total := total + Length(s) mod 7;
  end;
  Close(f);
  WriteLn(n, ' ', total);
  Reset(f);
  for i := 1 to 2999 do
    ReadLn(f);
  Read(f, s);
  WriteLn(s);
  Close(f);
end.
