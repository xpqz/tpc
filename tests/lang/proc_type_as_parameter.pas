program ProcTypeAsParameter;
{$F+}
{ a procedural parameter called inside the routine }
type TFn = function(x: Integer): Integer;
function Dbl(x: Integer): Integer; begin Dbl := 2 * x end;
function Inc1(x: Integer): Integer; begin Inc1 := x + 1 end;
function Apply(f: TFn; x: Integer): Integer;
begin
  Apply := f(x)
end;
begin
  WriteLn(Apply(Dbl, 20));
  WriteLn(Apply(Inc1, 20))
end.
