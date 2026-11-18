program SetInAggregates;
type
  TS = set of 0..15;
  Rec = record a: Integer; s: TS; b: Integer; end;
var r, q: Rec; arr: array[1..3] of TS; i, k, n: Integer;
function Count(x: Rec): Integer;
var k, n: Integer;
begin
  n := 0;
  for k := 0 to 15 do
    if k in x.s then n := n + 1;
  Count := n + x.a + x.b;
end;
begin
  r.a := 100; r.b := 200; r.s := [1, 3, 5];
  q := r;
  q.s := q.s + [7];
  WriteLn(Count(r), ' ', Count(q), ' ', r.a, ' ', q.b);
  for i := 1 to 3 do arr[i] := [i, i + 1..i + 3];
  for i := 1 to 3 do begin
    n := 0;
    for k := 0 to 15 do
      if k in arr[i] then n := n + 1;
    Write(n, ' ');
  end;
  WriteLn;
  WriteLn(arr[1] * arr[2] = [2, 3, 4], ' ', arr[3] <= arr[3] + arr[1])
end.
