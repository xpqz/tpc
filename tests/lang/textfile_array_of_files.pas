program TextfileArrayOfFiles;
var
  files: array[1..3] of Text;
  i, n: Integer;
  s: string;
  name: string[8];
begin
  for i := 1 to 3 do
  begin
    Str(i, name);
    Assign(files[i], 'PART' + name + '.TMP');
    Rewrite(files[i]);
  end;
  for n := 1 to 9 do
    WriteLn(files[(n - 1) mod 3 + 1], 'item ', n);
  for i := 1 to 3 do
  begin
    Close(files[i]);
    Reset(files[i]);
  end;
  for i := 1 to 3 do
  begin
    Write('file ', i, ':');
    while not Eof(files[i]) do
    begin
      ReadLn(files[i], s);
      Write(' ', s);
    end;
    WriteLn;
    Close(files[i]);
  end;
end.
