program StringEdgeMatrix;
var s, t: String[8]; i, n: Integer;
begin
  for i := -2 to 10 do
  begin
    for n := -1 to 9 do Write(Copy('abcdefgh', i, n), ',');
    WriteLn
  end;
  for i := -2 to 10 do
  begin
    for n := -1 to 9 do
    begin
      s := 'abcdefgh';
      Delete(s, i, n);
      Write(s, ',')
    end;
    WriteLn
  end;
  for i := -2 to 10 do
  begin
    t := 'abcdef';
    Insert('XY', t, i);
    Write(t, ',');
    t := 'abcdefgh';
    Insert('XY', t, i);
    Write(t, ',');
    t := '';
    Insert('XY', t, i);
    WriteLn(t)
  end;
  for i := 0 to 3 do
    WriteLn(Pos('', 'abc'), Pos('', ''), Pos('abc', 'abc'), Pos('abcd', 'abc'),
      Pos('c', 'abcabc'), Pos('ca', 'abcabc'), Pos('z', 'abc'), Pos('abc', ''))
end.
