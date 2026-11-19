program IoresultAppendToMissingFileFails;
{ Append, unlike Rewrite, needs the file to exist }
var
  f: Text;
  code: Integer;
begin
  {$I-}
  Assign(f, 'NEW.TXT');
  Append(f);
  code := IOResult;
  WriteLn('append: ', code);
  Reset(f);
  code := IOResult;
  WriteLn('reset: ', code);
  Rewrite(f);
  code := IOResult;
  WriteLn('rewrite: ', code);
  Close(f);
  Append(f);
  code := IOResult;
  WriteLn('append again: ', code);
  Close(f);
end.
