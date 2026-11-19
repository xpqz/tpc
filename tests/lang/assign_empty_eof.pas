program assign_empty_eof;
var
  f: Text;
  s: string;
  n: Integer;
begin
  Assign(f, '');
  Reset(f);
  n := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    n := n + 1;
    WriteLn(n, ':', s);
  end;
  WriteLn(Eof(f), Eoln(f));
  Close(f);
end.
