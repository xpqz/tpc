program FileReadAfterClose;
var
  f: Text;
  s: string;
begin
  Assign(f, 'ONE.TXT');
  Reset(f);
  ReadLn(f, s);
  WriteLn(s);
  Close(f);
  ReadLn(f, s);
  WriteLn('not reached');
end.
