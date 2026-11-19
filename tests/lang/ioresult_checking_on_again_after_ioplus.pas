program IoresultCheckingOnAgainAfterIoplus;
var
  f: Text;
  code: Integer;
  s: string;
begin
  {$I-}
  Assign(f, 'NONE.TXT');
  Reset(f);
  code := IOResult;
  WriteLn('code ', code);
  {$I+}
  Assign(f, 'SOME.TXT');
  Reset(f);
  ReadLn(f, s);
  WriteLn(s);
  Close(f);
end.
