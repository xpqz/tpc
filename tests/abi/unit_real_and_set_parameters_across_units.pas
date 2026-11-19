program UnitRealAndSetParametersAcrossUnits;
uses Maths;
var
  s: TSmallSet;
  r: Real;
begin
  r := Hypot2(3, 4);
  WriteLn(r:0:1);
  WriteLn(Half(7):0:2);
  s := [1, 3, 5];
  WriteLn(Count(s));
  Odds(s, 3);
  WriteLn(Count(s), ' ', 5 in s, ' ', 1 in s);
  Grow(s);
  WriteLn(Count(s));
end.
