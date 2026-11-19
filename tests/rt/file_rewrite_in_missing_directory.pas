program FileRewriteInMissingDirectory;
var
  f: Text;
begin
  WriteLn('before');
  Assign(f, 'NODIR\OUT.TXT');
  Rewrite(f);
  WriteLn('not reached');
end.
