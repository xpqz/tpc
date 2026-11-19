program IncludeInTheMiddleOfAConstantExpression;
const
  A = 1 + {$I TWO.INC} + 3;
begin
  WriteLn(A);
end.
