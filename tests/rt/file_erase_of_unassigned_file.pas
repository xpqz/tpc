program FileEraseOfUnassignedFile;
var
  f, g: Text;
begin
  WriteLn('before');
  Erase(f);
  WriteLn('not reached');
end.
