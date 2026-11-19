program FileRenameToNameWithDrive;
var
  f, g: Text;
begin
  WriteLn('before');
  Assign(f, 'ONE.TXT');
  Rename(f, 'A:TWO.TXT');
  WriteLn('not reached');
end.
