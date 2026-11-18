program ProcVarWithNestedCaller;
{$F+}
{ a nested routine calls through a procedural variable and a procedural parameter of the
  enclosing routine; the callee is a global routine }
type TF = function(x: Integer): Integer;
var viaGlobal: TF;
function Dbl(x: Integer): Integer; begin Dbl := x * 2 end;
function Neg(x: Integer): Integer; begin Neg := -x end;
procedure Outer(f: TF);
var acc: Integer;
  procedure Step(x: Integer);
  begin
    acc := acc + f(x) + viaGlobal(x)
  end;
begin
  acc := 0;
  Step(1);
  Step(10);
  WriteLn(acc)
end;
begin
  viaGlobal := Neg;
  Outer(Dbl);
  viaGlobal := Dbl;
  Outer(Neg)
end.
