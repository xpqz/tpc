program FileEraseOfConsoleFile;
var
  f, g: Text;
begin
  WriteLn('before');
  Erase(Input);
  WriteLn('not reached');
end.
