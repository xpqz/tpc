program ProcTypeFunctionWithParameters;
{$F+}
{ a function type with parameters, called through a variable }
type TBin = function(a, b: Integer): Integer;
var f: TBin;
function Add(a, b: Integer): Integer; begin Add := a + b end;
function Sub(a, b: Integer): Integer; begin Sub := a - b end;
begin
  f := Add;
  WriteLn(f(10, 3));
  f := Sub;
  WriteLn(f(10, 3))
end.
