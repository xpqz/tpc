program RealFuncParams;
function Half(x: Real): Real;
begin
  Half := x / 2;
end;
function Power(x: Real; n: Integer): Real;
begin
  if n = 0 then Power := 1
  else if Odd(n) then Power := x * Power(x, n - 1)
  else Power := Sqr(Power(x, n div 2));
end;
function Mean(a, b: Real): Real;
begin
  Mean := (a + b) / 2;
end;
procedure Scale(var x: Real; f: Real);
begin
  x := x * f;
end;
function Larger(a, b: Real): Real;
begin
  if a > b then Larger := a else Larger := b;
end;
var r: Real;
begin
  WriteLn(Half(5):0:2, ' ', Half(Half(1)):0:3);
  WriteLn(Power(2, 10):0:0, ' ', Power(0.5, 4):0:4, ' ', Power(3, 0):0:0, ' ', Power(-2, 3):0:0);
  WriteLn(Mean(1, 2):0:2, ' ', Mean(-3.5, 3.5):0:2);
  r := 3;
  Scale(r, 1.5);
  Scale(r, 2);
  WriteLn(r:0:2);
  WriteLn(Larger(1, 2.5):0:1, ' ', Larger(-1, -2):0:1, ' ', Larger(Half(10), 4):0:1)
end.
