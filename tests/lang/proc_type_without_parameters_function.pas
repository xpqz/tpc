program ProcTypeWithoutParametersFunction;
{$F+}
{ a parameterless function type is called without parentheses and used in expressions }
type TGet = function: Integer;
var g: TGet; total: Integer;
function Three: Integer; begin Three := 3 end;
function Nine: Integer; begin Nine := 9 end;
begin
  g := Three;
  total := g + g;
  g := Nine;
  total := total * 10 + g;
  WriteLn(total)
end.
