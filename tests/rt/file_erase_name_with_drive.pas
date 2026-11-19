program FileEraseNameWithDrive;
var
  f, g: Text;
begin
  WriteLn('before');
  Assign(f, 'A:ONE.TXT');
  Erase(f);
  WriteLn('not reached');
end.
