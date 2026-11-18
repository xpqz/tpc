program Sieve;
const N = 2000;
var comp: array[2..N] of Boolean; i, j, cnt, last: Integer;
begin
  for i := 2 to N do comp[i] := False;
  i := 2;
  while i * i <= N do begin
    if not comp[i] then begin
      j := i * i;
      while j <= N do begin comp[j] := True; j := j + i end
    end;
    i := i + 1
  end;
  cnt := 0;
  for i := 2 to N do
    if not comp[i] then begin cnt := cnt + 1; last := i; if cnt <= 15 then Write(i, ' ') end;
  WriteLn;
  WriteLn(cnt, ' ', last)
end.
