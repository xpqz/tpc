program UnitInterfaceConstTypeVarProc;
{ every kind of interface declaration is visible in the using program }
uses Shapes;
var
  r: TRect;
  c: TColor;
begin
  WriteLn(MaxSides);
  r.w := 3;
  r.h := 4;
  WriteLn(Area(r));
  WriteLn(Perimeter(r));
  c := Green;
  WriteLn(Ord(c));
  Counter := 10;
  Bump;
  Bump;
  WriteLn(Counter);
  WriteLn(Describe(c));
end.
