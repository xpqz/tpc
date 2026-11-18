program RealNeg;
var x: Real; k: Integer;
begin
  for k := -3 to 3 do begin
    x := k / 4;
    WriteLn(x:7:2, ' ', x:0:3);
  end;
  WriteLn(-1.0E5);
  WriteLn(-0.5E-3:0:6);
  WriteLn(-100.0:8:1)
end.
