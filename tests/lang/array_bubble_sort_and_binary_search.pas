program SortSearch;
const N = 20;
var a: array[1..N] of Integer; i, j, t, lo, hi, mid, key: Integer;
begin
  t := 17;
  for i := 1 to N do begin t := (t * 31 + 7) mod 101; a[i] := t end;
  for i := 1 to N - 1 do
    for j := 1 to N - i do
      if a[j] > a[j + 1] then begin t := a[j]; a[j] := a[j + 1]; a[j + 1] := t end;
  for i := 1 to N do Write(a[i], ' ');
  WriteLn;
  for t := 0 to 2 do begin
    key := a[t * 7 + 1];
    lo := 1; hi := N;
    while lo < hi do begin
      mid := (lo + hi) div 2;
      if a[mid] < key then lo := mid + 1 else hi := mid
    end;
    WriteLn(key, ' at ', lo)
  end
end.
