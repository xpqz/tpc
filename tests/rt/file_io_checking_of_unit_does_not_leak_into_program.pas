program FileIoCheckingOfUnitDoesNotLeakIntoProgram;
uses Quiet;
var
  f: Text;
begin
  WriteLn(Touch);
  Assign(f, 'MISSING.TXT');
  Reset(f);
  WriteLn('not reached');
end.
