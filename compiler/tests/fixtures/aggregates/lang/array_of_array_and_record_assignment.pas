program ArrayOfArrayAndRecordAssignment;
type Row = array[1..3] of Integer; Grid = array[1..2] of Row;
     Cell = record r: Row; tag: Char end; Cells = array[0..1] of Cell;
var g, h: Grid; cs: Cells; one: Cell; i, j: Integer;
begin
  for i := 1 to 2 do for j := 1 to 3 do g[i][j] := i * 10 + j;
  h := g;
  h[1] := h[2]; h[2][3] := -1;
  WriteLn(g[1][1], g[1][3], ' ', g[2][3], ' ', h[1][1], h[1][3], ' ', h[2][3]);
  g[1] := g[2]; g[2] := h[1];
  WriteLn(g[1][2], ' ', g[2][2], ' ', h[2][2]);
  one.r := g[1]; one.tag := 'k';
  cs[0] := one; cs[1] := cs[0]; cs[1].r[2] := 0; cs[0].tag := 'z';
  WriteLn(cs[0].r[2], cs[1].r[2], ' ', cs[0].tag, cs[1].tag, one.tag);
  g := g; cs[0] := cs[0];
  WriteLn(g[1][1], cs[0].r[1])
end.
