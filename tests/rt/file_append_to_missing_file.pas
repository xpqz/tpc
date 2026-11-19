program FileAppendToMissingFile;
var
  f: Text;
begin
  WriteLn('before');
  Assign(f, 'MISSING.TXT');
  Append(f);
  WriteLn('not reached');
end.
