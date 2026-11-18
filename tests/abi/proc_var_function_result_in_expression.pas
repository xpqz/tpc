program ProcVarFunctionResultInExpression;
{$F+}
{ indirect calls as operands, arguments and conditions }
type TF = function(x: Integer): Integer; TB = function(x: Integer): Boolean;
function Dbl(x: Integer): Integer; begin Dbl := x * 2 end;
function Inc1(x: Integer): Integer; begin Inc1 := x + 1 end;
function Even(x: Integer): Boolean; begin Even := not Odd(x) end;
var f, g: TF; e: TB; i: Integer;
begin
  f := Dbl; g := Inc1; e := Even;
  WriteLn(f(g(f(1))) + g(f(g(1))));
  for i := 0 to 5 do
    if e(i) and (f(i) > 2) then WriteLn(i, ' even and f > 2')
    else WriteLn(i, ' ', e(g(i)));
  WriteLn(f(i) * g(i))
end.
