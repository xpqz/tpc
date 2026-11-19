program FileReadFromFileNeverOpened;
var
  f: Text;
  s: string;
begin
  WriteLn('before');
  Assign(f, 'ONE.TXT');
  ReadLn(f, s);
  WriteLn('not reached');
end.
