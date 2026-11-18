program RealDefault;
var x: Real; k: Integer;
begin
  WriteLn(0.0);
  WriteLn(1.0);
  WriteLn(-1.0);
  WriteLn(0.5);
  WriteLn(0.25);
  WriteLn(0.125);
  WriteLn(100.0);
  WriteLn(1024.0);
  WriteLn(65536.0);
  WriteLn(1.0E6);
  WriteLn(1.0E10);
  WriteLn(12345.5);
  WriteLn(-0.0009765625);
  WriteLn(0.0009765625);
  x := 1;
  for k := 1 to 20 do x := x * 2;
  WriteLn(x);
  for k := 1 to 20 do x := x / 4;
  WriteLn(x);
  x := 1;
  for k := 1 to 100 do x := x * 2;
  WriteLn(x);
  WriteLn(x, x);
  WriteLn(1.0, ' ', 2.0, '|', 3.0)
end.
