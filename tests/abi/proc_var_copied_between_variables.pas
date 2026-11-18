program ProcVarCopiedBetweenVariables;
{$F+}
{ procedural values are copied by assignment and stay bound after the source changes }
type TF = function(x: Integer): Integer;
var f, g, h: TF;
function Dbl(x: Integer): Integer; begin Dbl := x * 2 end;
function Neg(x: Integer): Integer; begin Neg := -x end;
begin
  f := Dbl;
  g := f;
  f := Neg;
  h := g;
  WriteLn(f(5), ' ', g(5), ' ', h(5));
  g := f;
  WriteLn(g(9), ' ', h(9))
end.
