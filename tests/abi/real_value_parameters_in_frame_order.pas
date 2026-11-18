program RealParams;
function F(a: Integer; x: Real; b: Byte; y: Real; l: LongInt): Real;
begin
  F := a + x * b + y - l;
end;
function G(x: Real; n: Integer): Real;
begin
  if n = 0 then G := x else G := G(x * 2, n - 1) + 1;
end;
var r: Real;
begin
  r := F(1, 2.5, 4, 0.5, 100000);
  WriteLn(r:0:1);
  WriteLn(G(0.5, 10):0:1);
  WriteLn(F(-1, -0.5, 255, 0.25, -1):0:2)
end.
