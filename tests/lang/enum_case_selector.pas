program EnumCaseSelector;
type Op = (Add, Sub, Mul, Quot, Neg);
var o: Op; r: Integer;
function Apply(o: Op; a, b: Integer): Integer;
begin
  case o of
    Add: Apply := a + b;
    Sub: Apply := a - b;
    Mul: Apply := a * b;
    Quot: Apply := a div b
  else Apply := -a
  end
end;
begin
  for o := Add to Neg do WriteLn(Ord(o), ' ', Apply(o, 20, 4));
  for o := Add to Neg do
    case o of
      Add, Sub: Write('addsub ');
      Mul..Quot: Write('muldiv ');
    end;
  WriteLn
end.
