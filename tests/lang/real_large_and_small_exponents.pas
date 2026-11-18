program RealExp;
var x: Real; k: Integer;
begin
  x := 1;
  for k := 1 to 30 do x := x * 2;
  WriteLn(x);
  WriteLn(x:0:0);
  for k := 1 to 30 do x := x / 2;
  WriteLn(x:0:1);
  x := 1;
  for k := 1 to 40 do x := x / 2;
  WriteLn(x);
  x := 1;
  for k := 1 to 64 do x := x * 2;
  WriteLn(x)
end.
