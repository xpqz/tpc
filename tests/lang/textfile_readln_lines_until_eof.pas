program TextfileReadlnLinesUntilEof;
var
  f: Text;
  s: string;
  count, chars: Integer;
begin
  Assign(f, 'LINES.TXT');
  Reset(f);
  count := 0;
  chars := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    count := count + 1;
    chars := chars + Length(s);
    WriteLn(count, ' [', s, ']');
  end;
  Close(f);
  WriteLn(count, ' lines, ', chars, ' characters');
end.
