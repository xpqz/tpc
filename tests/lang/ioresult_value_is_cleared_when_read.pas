program IoresultValueIsClearedWhenRead;
var
  f: Text;
  first, second, third: Integer;
begin
  {$I-}
  Assign(f, 'NOPE.TXT');
  Reset(f);
  first := IOResult;
  second := IOResult;
  WriteLn(first, ' ', second);
  Reset(f);
  Reset(f);
  third := IOResult;
  WriteLn(third, ' ', IOResult);
  WriteLn(IOResult);
end.
