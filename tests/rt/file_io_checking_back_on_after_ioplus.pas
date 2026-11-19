program FileIoCheckingBackOnAfterIoplus;
var
  f: Text;
  code: Integer;
begin
  {$I-}
  Assign(f, 'MISSING.TXT');
  Reset(f);
  code := IOResult;
  WriteLn('checked: ', code);
  {$I+}
  Reset(f);
  WriteLn('not reached');
end.
