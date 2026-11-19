program TypedconstConstantExpressions;
const
  Base = 10;
  a: Integer = Base * 3 + 2;
  b: Integer = (Base - 4) div 2;
  c: Char = Chr(Ord('A') + 2);
  d: Integer = Ord('a');
  e: Boolean = Base > 5;
  f: Integer = -Base;
  g: LongInt = 1000 * 1000;
  h: Integer = Base mod 3;
  n: Integer = Length('hello');
begin
  WriteLn(a, ' ', b, ' ', c, ' ', d, ' ', e, ' ', f, ' ', g, ' ', h, ' ', n);
end.
