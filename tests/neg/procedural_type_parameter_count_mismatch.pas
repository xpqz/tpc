program NegTest;
{$F+}
type TFunc = function(x: Integer): Integer;
var f: TFunc;
function Add(a, b: Integer): Integer;
begin
  Add := a + b
end;
begin
  f := Add
end.
