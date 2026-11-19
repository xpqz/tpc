program UnitProceduralTypeInRecordAcrossUnits;
{$F+}
uses Shape, Kinds;
var
  shapes: array[1..3] of TShape;
  i: Integer;
begin
  MakeSquare(shapes[1], 3);
  MakeRect(shapes[2], 2, 5);
  MakeSquare(shapes[3], 1);
  for i := 1 to 3 do
    WriteLn(shapes[i].name, ' ', shapes[i].area(shapes[i].w, shapes[i].h));
end.
