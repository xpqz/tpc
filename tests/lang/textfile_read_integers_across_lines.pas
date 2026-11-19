program TextfileReadIntegersAcrossLines;
var
  f: Text;
  a, b, c, sum, i: Integer;
  big: LongInt;
begin
  Assign(f, 'NUMS.TXT');
  Reset(f);
  Read(f, a, b);
  WriteLn(a, ' ', b);
  ReadLn(f, c);
  WriteLn(c);
  Read(f, a);
  Read(f, b);
  Read(f, c);
  WriteLn(a, ' ', b, ' ', c);
  ReadLn(f, big);
  WriteLn(big);
  sum := 0;
  for i := 1 to 4 do
  begin
    Read(f, a);
    sum := sum + a;
  end;
  WriteLn(sum);
  Close(f);
end.
