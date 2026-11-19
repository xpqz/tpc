program FileResetOfUnassignedFile;
var
  f: Text;
begin
  WriteLn('before');
  Reset(f);
  WriteLn('not reached');
end.
