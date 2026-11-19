program NegTest;
{$F+}
type TFun = function(x: Integer): Integer;
var t: array[1..2] of TFun;
function Id(x: Integer): Integer;
begin
  Id := x
end;
begin
  t[1] := Id;
  t[1](1)
end.
