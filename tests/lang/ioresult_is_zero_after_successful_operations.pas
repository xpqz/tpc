program IoresultIsZeroAfterSuccessfulOperations;
var
  f: Text;
  s: string;
  n: Integer;
  total: Integer;
begin
  {$I-}
  total := 0;
  Assign(f, 'OK.TMP');
  Rewrite(f);
  total := total + IOResult;
  WriteLn(f, 'abc');
  total := total + IOResult;
  Close(f);
  total := total + IOResult;
  Reset(f);
  total := total + IOResult;
  ReadLn(f, s);
  total := total + IOResult;
  Close(f);
  total := total + IOResult;
  Append(f);
  total := total + IOResult;
  Close(f);
  n := IOResult;
  WriteLn(total, ' ', n);
end.
