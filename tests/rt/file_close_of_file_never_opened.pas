program FileCloseOfFileNeverOpened;
var
  f: Text;
begin
  WriteLn('before');
  Assign(f, 'ONE.TXT');
  Close(f);
  WriteLn('not reached');
end.
