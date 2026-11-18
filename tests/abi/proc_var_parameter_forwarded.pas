program ProcVarParameterForwarded;
{$F+}
{ a procedural parameter passed on to another routine and recursed with }
type TFunc = function(x: Integer): Integer;
function Dbl(x: Integer): Integer; begin Dbl := x * 2 end;
function Iter(f: TFunc; n, x: Integer): Integer;
begin
  if n = 0 then Iter := x
  else Iter := Iter(f, n - 1, f(x))
end;
function Outer(f: TFunc; x: Integer): Integer;
begin
  Outer := Iter(f, 5, x) + Iter(f, 0, x)
end;
begin
  WriteLn(Iter(Dbl, 10, 1));
  WriteLn(Outer(Dbl, 3))
end.
