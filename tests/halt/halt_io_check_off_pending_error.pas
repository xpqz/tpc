program H4;
var f: Text;
begin
  {$I-}
  Assign(f, 'NOSUCH.FIL');
  Reset(f);
  WriteLn(IOResult);
  WriteLn('open failed');
  Halt(0);
  WriteLn('not reached')
end.
