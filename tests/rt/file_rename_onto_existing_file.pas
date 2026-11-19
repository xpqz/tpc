program FileRenameOntoExistingFile;
var
  f, g: Text;
begin
  WriteLn('before');
  Assign(f, 'ONE.TXT');
  Rename(f, 'TWO.TXT');
  WriteLn('not reached');
end.
