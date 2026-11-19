program TypedconstArrayTwoDimensions;
const
  grid: array[1..3, 1..4] of Integer = (
    (1, 2, 3, 4),
    (5, 6, 7, 8),
    (9, 10, 11, 12));
  tri: array[0..2] of array[0..1] of Byte = ((1, 2), (3, 4), (5, 6));
var
  r, c: Integer;
begin
  for r := 1 to 3 do
  begin
    for c := 1 to 4 do
      Write(grid[r, c]:3);
    WriteLn;
  end;
  WriteLn(tri[2][1], ' ', tri[0][0]);
  grid[2, 3] := 0;
  WriteLn(grid[2, 3], ' ', grid[2][4]);
end.
