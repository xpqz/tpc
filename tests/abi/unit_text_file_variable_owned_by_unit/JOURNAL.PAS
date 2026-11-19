unit Journal;
interface
procedure OpenJournal;
procedure Note(n: Integer);
procedure CloseJournal;
procedure Replay;
implementation
var
  log: Text;
procedure OpenJournal;
begin
  Assign(log, 'JOURNAL.TXT');
  Rewrite(log);
end;
procedure Note(n: Integer);
begin
  WriteLn(log, 'entry ', n);
end;
procedure CloseJournal;
begin
  Close(log);
end;
procedure Replay;
var
  s: string;
begin
  Reset(log);
  while not Eof(log) do
  begin
    ReadLn(log, s);
    WriteLn('replay: ', s);
  end;
  Close(log);
end;
end.
