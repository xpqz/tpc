program ProcVarReassigned;
{$F+}
{ one variable bound to different routines of the same signature in turn }
type TOp = function(a, b: Integer): Integer;
var op: TOp; i: Integer;
function Add(a, b: Integer): Integer; begin Add := a + b end;
function Sub(a, b: Integer): Integer; begin Sub := a - b end;
function Mul(a, b: Integer): Integer; begin Mul := a * b end;
function Mx(a, b: Integer): Integer; begin if a > b then Mx := a else Mx := b end;
begin
  for i := 0 to 4 do begin
    case i mod 4 of
      0: op := Add;
      1: op := Sub;
      2: op := Mul;
      3: op := Mx
    end;
    WriteLn(i, ' ', op(12, 5))
  end
end.
