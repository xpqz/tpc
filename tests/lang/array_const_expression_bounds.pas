program ArrayConstBounds;
const N = 4; M = N * 2;
type TT = array[N..M, -N..-1] of Byte;
var t: TT; i, j, s: Integer;
begin
  s := 0;
  for i := N to M do for j := -N to -1 do begin t[i, j] := i + j; s := s + t[i, j] end;
  WriteLn(s, ' ', SizeOf(TT), ' ', t[M, -1], ' ', t[N, -N])
end.
