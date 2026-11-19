program IoresultMissingFileIsReported;
{ opening a file that does not exist, with I/O checking off }
var
  f: Text;
  code: Integer;
  s: string;
begin
  {$I-}
  Assign(f, 'NOTHERE.TXT');
  Reset(f);
  code := IOResult;
  if code <> 0 then
    WriteLn('cannot open NOTHERE.TXT, error ', code)
  else
  begin
    ReadLn(f, s);
    WriteLn(s);
    Close(f);
  end;
  Assign(f, 'HERE.TXT');
  Reset(f);
  code := IOResult;
  if code <> 0 then
    WriteLn('cannot open HERE.TXT, error ', code)
  else
  begin
    ReadLn(f, s);
    WriteLn('HERE.TXT says: ', s);
    Close(f);
  end;
  {$I+}
  WriteLn('done');
end.
