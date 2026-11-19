program AssignLeavesAPendingErrorAlone;
var
  f, g: Text;
  e: Integer;
begin
  {$I-}
  Assign(f, 'NONE.TXT');
  Reset(f);
  {$I+}
  Assign(g, 'OTHER.TXT');
  e := IOResult;
  {$I-}
  WriteLn('pending was ', e);
end.
