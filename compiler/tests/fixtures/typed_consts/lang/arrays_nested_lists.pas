program ArraysNestedLists;
type
  TRow = array[1..3] of Integer;
const
  grid: array[1..2, 1..3] of Integer = ((1, 2, 3), (4, 5, 6));
  rows: array[0..1] of TRow = ((7, 8, 9), (10, 11, 12));
  cube: array[0..1, 0..1, 0..1] of Byte = (((1, 2), (3, 4)), ((5, 6), (7, 8)));
  flags: array[Boolean] of string[5] = ('false', 'true');
  single: array[5..5] of Integer = (42);
  longs: array[1..3] of LongInt = (-100000, 0, 100000);
  letters: array['a'..'c'] of Integer = (1, 2, 3);
var
  i, j, sum: Integer;
begin
  sum := 0;
  for i := 1 to 2 do
    for j := 1 to 3 do
      sum := sum + grid[i, j] * i;
  WriteLn(sum, ' ', rows[1][2], ' ', rows[0, 3], ' ', cube[1, 0, 1], ' ', cube[0][1][0]);
  WriteLn(flags[False], flags[True], ' ', single[5], ' ', longs[1], ' ', longs[3], ' ', letters['b']);
  grid[2, 3] := 60;
  cube[1, 1, 1] := 80;
  rows[0] := rows[1];
  WriteLn(grid[2, 3], ' ', cube[1, 1, 1], ' ', rows[0, 1]);
end.
