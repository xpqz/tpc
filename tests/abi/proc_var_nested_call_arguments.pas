program ProcVarNestedCallArguments;
{$F+}
{ indirect calls whose arguments are indirect calls, through different variables, in one
  expression with direct calls; arguments are converted to the parameter types }
type TF = function(x: Integer): Integer;
type TB = function(b: Byte; l: LongInt): LongInt;
var f, g: TF; mix: TB;
function Inc1(x: Integer): Integer; begin Inc1 := x + 1 end;
function Dbl(x: Integer): Integer; begin Dbl := x * 2 end;
function Mix2(b: Byte; l: LongInt): LongInt; begin Mix2 := l * 1000 + b end;
begin
  f := Inc1; g := Dbl; mix := Mix2;
  WriteLn(f(g(f(1))), ' ', g(f(g(f(1)))));
  WriteLn(mix(f(254), g(f(1)) * 100000), ' ', Mix2(g(2), f(g(3))) + mix(1, 2));
  WriteLn(f(f(f(f(32764)))))
end.
