program RecordList;
type Node = record val, next: Integer end;
var pool: array[1..6] of Node; head, p, i: Integer;
begin
  for i := 1 to 6 do begin pool[i].val := i * 100; pool[i].next := i - 1 end;
  head := 6;
  p := head;
  while p <> 0 do begin Write(pool[p].val, ' '); p := pool[p].next end;
  WriteLn;
  pool[3].next := 1;
  p := head;
  while p <> 0 do begin Write(pool[p].val, ' '); p := pool[p].next end;
  WriteLn
end.
