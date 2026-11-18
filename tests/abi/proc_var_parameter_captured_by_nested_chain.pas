program ProcVarParameterCapturedByNestedChain;
{$F+}
{ a procedural value parameter captured two levels down: the nested routines call it and pass
  it on to a global routine; the global variable it also modifies is seen at every depth }
type TF = function(x: Integer): Integer;
var calls: Integer;
function Dbl(x: Integer): Integer; begin calls := calls + 1; Dbl := x * 2 end;
function Add3(x: Integer): Integer; begin calls := calls + 1; Add3 := x + 3 end;
function Twice(p: TF; x: Integer): Integer;
begin
  Twice := p(p(x))
end;
function Outer(f: TF; x: Integer): Integer;
  function Mid(y: Integer): Integer;
    function Inner(z: Integer): Integer;
    begin
      Inner := Twice(f, z) + f(z)
    end;
  begin
    Mid := Inner(y) + Inner(y + 1)
  end;
begin
  Outer := Mid(x)
end;
begin
  calls := 0;
  WriteLn(Outer(Dbl, 1), ' ', calls);
  WriteLn(Outer(Add3, 1), ' ', calls)
end.
