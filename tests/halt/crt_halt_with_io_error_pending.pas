program H11;
uses Crt;
var f: Text;
begin
  {$I-}
  Assign(f, 'NOSUCH.FIL');
  Reset(f);
  WriteLn(IOResult);
  WriteLn('open failed');
  Halt(6)
end.
