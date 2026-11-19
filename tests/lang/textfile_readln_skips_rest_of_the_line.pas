program TextfileReadlnSkipsRestOfTheLine;
var
  f: Text;
  a, b: Integer;
  s: string;
begin
  Assign(f, 'SKIP.TXT');
  Reset(f);
  ReadLn(f, a);
  ReadLn(f, b);
  ReadLn(f, s);
  WriteLn(a, ' ', b, ' [', s, ']');
  ReadLn(f);
  ReadLn(f, s);
  WriteLn('[', s, ']');
  Close(f);
end.
