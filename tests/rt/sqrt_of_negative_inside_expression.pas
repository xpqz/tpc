program SqrtNegExpr;
var
  X: Real;
begin
  X := 0.5;
  WriteLn(Sqrt(X * X):0:4);
  WriteLn(Sqrt(X - 1) + 1:0:4);
  WriteLn('after');
end.
