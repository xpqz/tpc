program ReportNumbersSurviveARoundTripThroughAFile;
var
  f: Text;
  i, back, bad: Integer;
  l, lback: LongInt;
  r, rback: Real;
begin
  Assign(f, 'ROUND.TMP');
  Rewrite(f);
  for i := -3 to 3 do
    WriteLn(f, i * 10000, ' ', i * 100000, ' ', i * 0.25:0:2);
  Close(f);
  Reset(f);
  bad := 0;
  for i := -3 to 3 do
  begin
    ReadLn(f, back, lback, rback);
    if back <> i * 10000 then bad := bad + 1;
    if lback <> i * 100000 then bad := bad + 1;
    if rback <> i * 0.25 then bad := bad + 1;
  end;
  Close(f);
  WriteLn('mismatches: ', bad);
end.
