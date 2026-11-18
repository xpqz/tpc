program RealConsts;
const
  Half = 0.5;
  Big = 1.0E3;
  Small = 2.5E-1;
  Neg = -1.25;
  Three = 3.0;
  Exp1 = 1E2;
  Sum = Half + 0.25;
var x: Real;
begin
  WriteLn(Half:0:2, ' ', Big:0:1, ' ', Small:0:3, ' ', Neg:0:2, ' ', Three:0:0);
  x := Big * Small;
  WriteLn(x:0:1);
  WriteLn(Exp1:0:0, ' ', 5E0:0:0, ' ', 1.5e1:0:1, ' ', 0.5E+1:0:1, ' ', 25E-2:0:2);
  WriteLn(Sum:0:2)
end.
