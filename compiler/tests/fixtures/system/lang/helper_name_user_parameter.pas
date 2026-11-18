program HelperNameUserParameter;
const AbsLong = 3;
function Use(AbsLong: Integer): Integer;
var Abslong2: Integer;
begin
  Abslong2 := -AbsLong;
  Use := Abs(Abslong2) + AbsLong
end;
begin
  WriteLn(Use(-8), ' ', Abs(-AbsLong))
end.
