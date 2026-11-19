program FileRenameOfMissingFile;
var
  f, g: Text;
begin
  WriteLn('before');
  Assign(f, 'MISSING.TXT');
  Rename(f, 'NEW.TXT');
  WriteLn('not reached');
end.
