program ProcVarReturningProceduralValueBySelection;
{$F+}
{ a function selects a routine by number through a Case and the caller applies it }
type TF = function(x: Integer): Integer;
function Dbl(x: Integer): Integer; begin Dbl := x * 2 end;
function Neg(x: Integer): Integer; begin Neg := -x end;
function Sq(x: Integer): Integer; begin Sq := x * x end;
procedure Pick(n: Integer; var f: TF);
begin
  case n of
    0: f := Dbl;
    1: f := Neg
  else
    f := Sq
  end
end;
var f: TF; i: Integer;
begin
  for i := 0 to 3 do begin
    Pick(i, f);
    Write(f(7), ' ')
  end;
  WriteLn
end.
