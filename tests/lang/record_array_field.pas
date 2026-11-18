program RecordArrayField;
type RT = record cnt: array[1..5] of Integer; m: array[0..1, 0..1] of Byte; n: Integer end;
var r: RT; i: Integer;
begin
  for i := 1 to 5 do r.cnt[i] := i * i;
  r.m[0, 0] := 1; r.m[0, 1] := 2; r.m[1, 0] := 3; r.m[1, 1] := 4; r.n := 5;
  WriteLn(r.cnt[5], ' ', r.cnt[1], ' ', r.m[1][0], ' ', r.m[1, 1], ' ', r.n, ' ', SizeOf(RT))
end.
