program RtNested;
var r: Real;
function Inv(x: Real): Real;
begin
  Inv := 1 / x
end;
function Twice(x: Real): Real;
begin
  Twice := 2 * Inv(x - 1)
end;
begin
  WriteLn(Twice(3):0:2);
  WriteLn(Twice(1.5):0:2);
  r := Twice(1) + 1;
  WriteLn('not reached')
end.
