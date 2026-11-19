program MathValues;
var
  r: Real;
  i: Integer;
  l: LongInt;
begin
  WriteLn(Exp(0):0:8, ' ', Exp(1):0:8, ' ', Exp(-1):0:8, ' ', Exp(2.5):0:8);
  WriteLn(Exp(10):0:4, ' ', Exp(-10):0:10, ' ', Exp(0.5):0:8);
  WriteLn(Ln(1):0:8, ' ', Ln(2):0:8, ' ', Ln(10):0:8, ' ', Ln(0.5):0:8, ' ', Ln(1000):0:8);
  WriteLn(Ln(1E10):0:8, ' ', Ln(1E-10):0:8);
  WriteLn(Sin(0):0:8, ' ', Sin(1):0:8, ' ', Sin(-1):0:8, ' ', Sin(0.5):0:8, ' ', Sin(3):0:8);
  WriteLn(Cos(0):0:8, ' ', Cos(1):0:8, ' ', Cos(-1):0:8, ' ', Cos(0.5):0:8, ' ', Cos(3):0:8);
  WriteLn(ArcTan(0):0:8, ' ', ArcTan(1):0:8, ' ', ArcTan(-1):0:8, ' ', ArcTan(0.5):0:8);
  WriteLn(ArcTan(100):0:8, ' ', ArcTan(-1000):0:8, ' ', ArcTan(1E6):0:8);
  WriteLn(Pi:0:10, ' ', Pi * 2:0:10, ' ', -Pi:0:10, ' ', Pi / 4:0:10);
  WriteLn(Sin(Pi / 2):0:8, ' ', Cos(Pi):0:8, ' ', Sin(Pi / 6):0:8, ' ', Cos(Pi / 3):0:8);
  WriteLn(Sin(Pi / 4):0:8, ' ', Cos(Pi / 4):0:8, ' ', ArcTan(1) * 4:0:10);
  WriteLn(Sin(2 * Pi - 1):0:8, ' ', Cos(5):0:8, ' ', Sin(10):0:8, ' ', Cos(-7):0:8);
  WriteLn(Sin(100):0:8, ' ', Cos(50):0:8, ' ', Sin(-30):0:8);
  { The argument may be any integer type, and the result is a Real. }
  i := 2; l := 3;
  WriteLn(Exp(i):0:8, ' ', Ln(i):0:8, ' ', Sin(i):0:8, ' ', Cos(l):0:8, ' ', ArcTan(l):0:8);
  r := Exp(Ln(5)); WriteLn(r:0:6);
  r := Sqr(Sin(0.7)) + Sqr(Cos(0.7)); WriteLn(r:0:10);
  r := Exp(Ln(123.456) + Ln(2)); WriteLn(r:0:6);
  r := Sin(ArcTan(1)) * Sin(ArcTan(1)); WriteLn(r:0:10);
  r := Exp(-0.0); WriteLn(r:0:2);
  WriteLn(Trunc(Pi * 1000), ' ', Round(Exp(3)), ' ', Trunc(Ln(100) * 100));
  WriteLn(Abs(Sin(-2)):0:8, ' ', Sqrt(Exp(2)):0:8, ' ', Exp(Sqrt(4)):0:8);
end.
