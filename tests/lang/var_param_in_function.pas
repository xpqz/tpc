program VarParamInFunction;
{ a function with a var parameter returns a value and changes the argument }
function Take(var budget: Integer; want: Integer): Integer;
var got: Integer;
begin
  if want > budget then got := budget else got := want;
  budget := budget - got;
  Take := got
end;
var b: Integer;
begin
  b := 10;
  WriteLn(Take(b, 4), ' ', b);
  WriteLn(Take(b, 4), ' ', b);
  WriteLn(Take(b, 4), ' ', b);
  WriteLn(Take(b, 4), ' ', b)
end.
