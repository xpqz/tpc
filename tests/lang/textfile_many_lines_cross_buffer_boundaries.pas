program TextfileManyLinesCrossBufferBoundaries;
{ enough text to fill any internal buffer many times }
var
  f: Text;
  i, n, total, expected: Integer;
  s: string;
  num: string[6];
begin
  Assign(f, 'BIG.TMP');
  Rewrite(f);
  expected := 0;
  for i := 1 to 1500 do
  begin
    Str(i, num);
    s := 'line number ' + num;
    expected := expected + Length(s);
    WriteLn(f, s);
  end;
  Close(f);
  Reset(f);
  n := 0;
  total := 0;
  while not Eof(f) do
  begin
    ReadLn(f, s);
    n := n + 1;
    total := total + Length(s);
  end;
  Close(f);
  WriteLn(n, ' ', total, ' ', total = expected);
  Reset(f);
  for i := 1 to 1499 do
    ReadLn(f);
  ReadLn(f, s);
  WriteLn(s);
  Close(f);
end.
