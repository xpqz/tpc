program IoresultWriteAfterFailedOpenWithOutputToConsole;
var
  f: Text;
  code: Integer;
begin
  {$I-}
  Assign(f, 'NODIR\X.TXT');
  Rewrite(f);
  code := IOResult;
  WriteLn('rewrite: ', code);
  WriteLn(f, 'lost');
  code := IOResult;
  WriteLn('write to unopened file: ', code);
  Close(f);
  code := IOResult;
  WriteLn('close: ', code);
end.
