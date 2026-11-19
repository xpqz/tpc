program TextfileWritelnWithoutArgumentsWritesEmptyLines;
var
  f: Text;
  s: string;
  n, empties: Integer;
begin
  Assign(f, 'GAPS.TXT');
  Rewrite(f);
  WriteLn(f);
  WriteLn(f, 'a');
  WriteLn(f);
  WriteLn(f);
  Write(f, 'no end of line');
  Close(f);
  Reset(f);
  n := 0;
  empties := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    n := n + 1;
    if s = '' then empties := empties + 1;
  end;
  Close(f);
  WriteLn(n, ' lines, ', empties, ' empty');
end.
