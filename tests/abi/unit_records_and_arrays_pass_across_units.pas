program UnitRecordsAndArraysPassAcrossUnits;
uses Vec;
var
  a, b, c: TVec;
  v: TVector;
  i: Integer;
begin
  a.x := 1; a.y := 2;
  b.x := 10; b.y := 20;
  Sum(a, b, c);
  WriteLn(c.x, ' ', c.y);
  Scale(c, 3);
  WriteLn(c.x, ' ', c.y);
  WriteLn(a.x, ' ', a.y);
  WriteLn(Dot(a, b));
  for i := 1 to 4 do v[i] := i;
  WriteLn(Total(v));
  WriteLn(v[1]);
  Reverse(v);
  for i := 1 to 4 do Write(v[i], ' ');
  WriteLn;
  WriteLn(Total(v));
end.
