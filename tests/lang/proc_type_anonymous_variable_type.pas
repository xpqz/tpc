program ProcTypeAnonymousVariableType;
{$F+}
{ procedural type written directly in a var declaration }
var f: function(x: Integer): Integer; g: procedure;
function Neg(x: Integer): Integer; begin Neg := -x end;
procedure Say; begin WriteLn('say') end;
begin
  f := Neg; g := Say;
  g;
  WriteLn(f(8))
end.
