program RealLoops;
var x, sum: Real; k: Integer;
begin
  sum := 0;
  for k := 1 to 8 do sum := sum + 0.5;
  WriteLn(sum:0:2);
  sum := 0;
  x := 1;
  for k := 1 to 10 do begin
    sum := sum + x;
    x := x / 2;
  end;
  WriteLn(sum:0:10);
  x := 1;
  for k := 1 to 16 do x := x * 2;
  WriteLn(x:0:0);
  sum := 0;
  for k := 1 to 100 do sum := sum + k;
  WriteLn(sum:0:0);
  sum := 1;
  k := 0;
  while sum < 1000 do begin
    sum := sum * 1.5;
    k := k + 1;
  end;
  WriteLn(k);
  x := 10;
  repeat
    x := x / 2;
  until x < 0.2;
  WriteLn(x:0:5)
end.
