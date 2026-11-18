program HelperNameUserVariable;
var AbsLong: Integer; w: Word;
function Neg(x: Integer): Integer;
begin
  Neg := Abs(x)
end;
begin
  AbsLong := -9;
  w := 40000;
  WriteLn(Abs(AbsLong), ' ', AbsLong, ' ', Neg(AbsLong), ' ', Abs(w), ' ', Abs(-32768));
  WriteLn(Abs(AbsLong) + AbsLong)
end.
