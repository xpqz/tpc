program TextfileTwoFilesOpenAtOnce;
var
  src, dst: Text;
  s: string;
  n: Integer;
begin
  Assign(src, 'SRC.TXT');
  Assign(dst, 'COPY.TXT');
  Reset(src);
  Rewrite(dst);
  n := 0;
  while not Eof(src) do
  begin
    ReadLn(src, s);
    n := n + 1;
    WriteLn(dst, n, ': ', s);
  end;
  Close(src);
  Close(dst);
  Reset(dst);
  while not Eof(dst) do
  begin
    ReadLn(dst, s);
    WriteLn(s);
  end;
  Close(dst);
end.
