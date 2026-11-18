program ProcVarNestedRoutineAssignsCapturedVariable;
{$F+}
{ a procedural variable captured by a nested routine lives in the outer frame: the nested
  routine assigns it a global routine and calls it, the outer routine calls it afterwards }
type TF = function(x: Integer): Integer;
function Dbl(x: Integer): Integer; begin Dbl := x * 2 end;
function Neg(x: Integer): Integer; begin Neg := -x end;
procedure Outer(first: Boolean);
var f: TF; total: Integer;
  procedure Choose;
  begin
    if first then f := Dbl else f := Neg;
    total := total + f(10)
  end;
  procedure Again;
  begin
    total := total + f(1)
  end;
begin
  total := 0;
  Choose;
  Again;
  total := total + f(100);
  WriteLn(total)
end;
begin
  Outer(True);
  Outer(False)
end.
