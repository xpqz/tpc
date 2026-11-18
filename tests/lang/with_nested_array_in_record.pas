program WithNestedArray;
type RT = record m: array[1..2] of record p, q: Integer end; z: Integer end;
var r: RT; i: Integer;
begin
  with r do begin
    for i := 1 to 2 do with m[i] do begin p := i; q := i * 10 end;
    z := m[1].q + m[2].q
  end;
  WriteLn(r.m[1].p, r.m[1].q, ' ', r.m[2].p, r.m[2].q, ' ', r.z)
end.
