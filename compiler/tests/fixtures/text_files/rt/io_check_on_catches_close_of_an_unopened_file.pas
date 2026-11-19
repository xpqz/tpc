program IoCheckOnCatchesCloseOfAnUnopenedFile;
var
  f: Text;
  e: Integer;
begin
  {$I-}
  Assign(f, 'NONE.TXT');
  Reset(f);
  e := IOResult;
  {$I+}
  WriteLn('ok');
  Close(f);
  WriteLn('not reached');
end.
