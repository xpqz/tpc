program FileRenameOfUnassignedFile;
var
  f, g: Text;
begin
  WriteLn('before');
  Rename(f, 'NEW.TXT');
  WriteLn('not reached');
end.
