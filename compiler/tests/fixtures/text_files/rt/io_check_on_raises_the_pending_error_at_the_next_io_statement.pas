program IoCheckOnRaisesThePendingErrorAtTheNextIoStatement;
var
  f: Text;
  e: Integer;
begin
  {$I-}
  Assign(f, 'NONE.TXT');
  Reset(f);
  {$I+}
  e := 1;
  WriteLn('shown?');
  WriteLn('not reached');
end.
