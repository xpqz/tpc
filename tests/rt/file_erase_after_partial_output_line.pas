program FileEraseAfterPartialOutputLine;
var
  f, g: Text;
begin
  WriteLn('before');
  Write('partial');
  Assign(f, 'MISSING.TXT');
  Erase(f);
  WriteLn('not reached');
end.
