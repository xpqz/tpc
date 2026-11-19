program FileWriteToFileOpenForInput;
var
  f: Text;
begin
  Assign(f, 'ONE.TXT');
  Reset(f);
  WriteLn('reading');
  WriteLn(f, 'x');
  WriteLn('not reached');
end.
