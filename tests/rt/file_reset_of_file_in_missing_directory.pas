program FileResetOfFileInMissingDirectory;
var
  f: Text;
begin
  WriteLn('before');
  Assign(f, 'NODIR\DATA.TXT');
  Reset(f);
  WriteLn('not reached');
end.
