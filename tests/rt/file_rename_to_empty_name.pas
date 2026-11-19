program FileRenameToEmptyName;
var
  f, g: Text;
begin
  WriteLn('before');
  Assign(f, 'ONE.TXT');
  Rename(f, '');
  WriteLn('not reached');
end.
