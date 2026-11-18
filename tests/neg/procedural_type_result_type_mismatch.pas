program NegTest;
{$F+}
type TFunc = function(x: Integer): Integer;
var f: TFunc;
function Wide(x: Integer): LongInt;
begin
  Wide := 0
end;
begin
  f := Wide
end.
