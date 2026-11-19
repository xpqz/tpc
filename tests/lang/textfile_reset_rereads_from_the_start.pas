program TextfileResetRereadsFromTheStart;
var
  f: Text;
  a, b: string;
begin
  Assign(f, 'TWO.TXT');
  Reset(f);
  ReadLn(f, a);
  ReadLn(f, b);
  WriteLn(a, b);
  Reset(f);
  ReadLn(f, a);
  WriteLn(a);
  Close(f);
  Reset(f);
  ReadLn(f, a);
  WriteLn(a);
  Close(f);
end.
