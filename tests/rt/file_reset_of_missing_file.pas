program FileResetOfMissingFile;
var
  f: Text;
begin
  WriteLn('before');
  Assign(f, 'MISSING.TXT');
  Reset(f);
  WriteLn('not reached');
end.
