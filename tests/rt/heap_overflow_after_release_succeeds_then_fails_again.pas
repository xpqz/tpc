program HeapRelease;
var m, p: Pointer; i: Integer;
begin
  Mark(m);
  for i := 1 to 2 do begin
    WriteLn('round ', i);
    while MaxAvail >= 30000 do GetMem(p, 30000);
    Release(m);
  end;
  WriteLn('exhausting');
  while True do GetMem(p, 30000);
end.
