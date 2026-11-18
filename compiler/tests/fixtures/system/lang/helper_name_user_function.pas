program HelperNameUserFunction;
var i: Integer; l: LongInt;
function AbsLong(x: LongInt): LongInt;
begin
  AbsLong := 42
end;
begin
  i := -5;
  l := -2147483647;
  WriteLn(Abs(i), ' ', Abs(-7), ' ', Abs(l), ' ', Abs(i + 1), ' ', Abs(Abs(-3)));
  WriteLn(AbsLong(-1), ' ', Abs(AbsLong(-1)))
end.
