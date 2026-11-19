program FileEraseOfMissingFile;
var
  f, g: Text;
begin
  WriteLn('before');
  Assign(f, 'MISSING.TXT');
  Erase(f);
  WriteLn('not reached');
end.
