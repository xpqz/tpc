program TextTypeAliasAndRecordsWithFiles;
type
  TFile = Text;
  TLog = record
    name: string[12];
    out: TFile;
    lines: Integer;
  end;
var
  logs: array[1..3] of TLog;
  i, k: Integer;
  f: TFile;
  s: string;
begin
  for i := 1 to 3 do
  begin
    logs[i].name := 'LOG' + Chr(48 + i) + '.TXT';
    logs[i].lines := 0;
    Assign(logs[i].out, logs[i].name);
    Rewrite(logs[i].out);
  end;
  for k := 1 to 7 do
  begin
    i := k mod 3 + 1;
    WriteLn(logs[i].out, 'entry ', k);
    logs[i].lines := logs[i].lines + 1;
  end;
  for i := 1 to 3 do
    Close(logs[i].out);
  for i := 1 to 3 do
  begin
    Assign(f, logs[i].name);
    Reset(f);
    Write(logs[i].name, ' has ', logs[i].lines, ':');
    while not Eof(f) do
    begin
      ReadLn(f, s);
      Write(' ', s);
    end;
    WriteLn;
    Close(f);
  end;
end.
