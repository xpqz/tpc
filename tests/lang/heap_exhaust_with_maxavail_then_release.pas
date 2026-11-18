program HeapExhaust;
var m, p: Pointer; before, n: LongInt; round: Integer;
begin
  Mark(m);
  before := MemAvail;
  for round := 1 to 3 do begin
    n := 0;
    while MaxAvail >= 4096 do begin
      GetMem(p, 4096);
      n := n + 1;
    end;
    WriteLn(round, ' ', n > 5, ' ', MaxAvail < 4096, ' ', MemAvail < before);
    Release(m);
    WriteLn(MemAvail = before, ' ', MaxAvail >= 4096)
  end
end.
