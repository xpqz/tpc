program ReportSortedRecordsWrittenAndReadBack;
type
  TEntry = record
    name: string[10];
    score: Integer;
  end;
var
  entries: array[1..10] of TEntry;
  n, i, j, comma, code: Integer;
  line, field: string;
  tmp: TEntry;
  f: Text;
begin
  Assign(f, 'SCORES.TXT');
  Reset(f);
  n := 0;
  while not Eof(f) do
  begin
    ReadLn(f, line);
    comma := Pos(',', line);
    n := n + 1;
    entries[n].name := Copy(line, 1, comma - 1);
    field := Copy(line, comma + 1, 255);
    Val(field, entries[n].score, code);
  end;
  Close(f);
  for i := 1 to n - 1 do
    for j := 1 to n - i do
      if entries[j].score < entries[j + 1].score then
      begin
        tmp := entries[j];
        entries[j] := entries[j + 1];
        entries[j + 1] := tmp;
      end;
  Assign(f, 'RANKED.TXT');
  Rewrite(f);
  for i := 1 to n do
    WriteLn(f, i, '. ', entries[i].name, ' (', entries[i].score, ')');
  Close(f);
  Reset(f);
  while not Eof(f) do
  begin
    ReadLn(f, line);
    WriteLn(line);
  end;
  Close(f);
end.
