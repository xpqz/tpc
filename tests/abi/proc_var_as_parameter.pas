program ProcVarAsParameter;
{$F+}
{ procedural values passed as value parameters and called inside }
type TFunc = function(x: Integer): Integer;
function Sq(x: Integer): Integer; begin Sq := x * x end;
function Inc1(x: Integer): Integer; begin Inc1 := x + 1 end;
function Apply(f: TFunc; x: Integer): Integer;
begin
  Apply := f(x)
end;
function Twice(f: TFunc; x: Integer): Integer;
begin
  Twice := f(f(x))
end;
var g: TFunc;
begin
  WriteLn(Apply(Sq, 7));
  WriteLn(Apply(Inc1, 7));
  WriteLn(Twice(Sq, 3));
  WriteLn(Twice(Inc1, 3));
  g := Sq;
  WriteLn(Apply(g, 12), ' ', Twice(g, 2))
end.
