program IoresultPendingErrorSkipsFollowingIo;
{ while an I/O error is pending, later I/O calls (console output included) do nothing }
var
  f: Text;
  code: Integer;
begin
  {$I-}
  Assign(f, 'MISSING.TXT');
  Reset(f);
  Write('A');
  WriteLn('B');
  code := IOResult;
  WriteLn('code ', code);
  Write('C');
  WriteLn('D');
  Reset(f);
  WriteLn('E');
  WriteLn(IOResult);
  WriteLn('F');
end.
