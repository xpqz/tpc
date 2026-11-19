program RealLiterals;
const
  Half = 0.5;
  Neg = -2.5;
  Big = 1.0E10;
  Tiny = 0.0009765625;
  Scaled = 1.25E2;
  Whole = 7.0;
  NoDot = 3E2;
var x: Real;
begin
  WriteLn(Half:0:3);
  WriteLn(Neg:0:3);
  WriteLn(Big:0:0);
  WriteLn(Tiny:0:10);
  WriteLn(Scaled:0:2);
  WriteLn(Whole:0:1);
  WriteLn(NoDot:0:1);
  x := Half + Neg;
  WriteLn(x:0:2);
  x := Big / 4;
  WriteLn(x:0:1);
  x := 1.5E+2;
  WriteLn(x:0:1);
  x := 2.5E-1;
  WriteLn(x:0:2);
  x := 00012.50;
  WriteLn(x:0:2);
  x := 1.0E0;
  WriteLn(x:0:0);
  WriteLn(0.5:0:1, ' ', 12.25:6:2, ' ', 1.0E3:0:0)
end.
