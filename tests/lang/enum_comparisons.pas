program EnumComparisons;
type Day = (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
var a, b: Day;
begin
  a := Tue; b := Fri;
  WriteLn(a < b, ' ', a > b, ' ', a <= b, ' ', a >= b, ' ', a = b, ' ', a <> b);
  WriteLn(a = Tue, ' ', b <> Fri, ' ', Mon < Sun, ' ', Sun > Sat, ' ', Sun <= Sun, ' ', Mon >= Tue);
  b := a;
  WriteLn(a = b, ' ', a <= b, ' ', a >= b, ' ', a < b)
end.
