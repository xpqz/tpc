program TextfileReadRealNumbers;
var
  f: Text;
  r, total: Real;
  i: Integer;
begin
  Assign(f, 'REALS.TXT');
  Reset(f);
  total := 0;
  for i := 1 to 6 do
  begin
    Read(f, r);
    WriteLn(r:0:3);
    total := total + r;
  end;
  WriteLn(total:0:3);
  Close(f);
end.
