{ Reads a file of the directory, writes one, erases one and renames one. }
var
  f: Text;
  line: String;
begin
  Assign(f, 'EXISTING.TXT');
  Reset(f);
  ReadLn(f, line);
  Close(f);
  WriteLn('read ', line);

  Assign(f, 'CREATED.TXT');
  Rewrite(f);
  WriteLn(f, 'made by the program');
  Close(f);

  Assign(f, 'OLD.TXT');
  Erase(f);

  Assign(f, 'FROM.TXT');
  Rename(f, 'TO.TXT');
  WriteLn('done');
end.
