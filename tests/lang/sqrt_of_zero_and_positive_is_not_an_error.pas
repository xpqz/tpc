program SqrtZero;
var
  X: Real;
begin
  X := 0;
  WriteLn(Sqrt(X):0:4, ' ', Sqrt(0):0:4, ' ', Sqrt(-X):0:4);
  WriteLn(Sqrt(2):0:6, ' ', Sqrt(16):0:1, ' ', Sqrt(1e-6):0:6);
  WriteLn('done');
end.
