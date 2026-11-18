program PtrStress;
type
  PNode = ^Node;
  Node = record
    a, b: LongInt;
    next: PNode;
  end;
var head, n: PNode; i, round, cnt: Integer; before: LongInt; ok: Boolean;
begin
  before := MemAvail;
  ok := True;
  for round := 1 to 20 do begin
    head := nil;
    for i := 1 to 200 do begin
      New(n);
      n^.a := i;
      n^.b := round;
      n^.next := head;
      head := n;
    end;
    cnt := 0;
    n := head;
    while n <> nil do begin
      if (n^.a + cnt <> 200) or (n^.b <> round) then ok := False;
      cnt := cnt + 1;
      n := n^.next;
    end;
    if cnt <> 200 then ok := False;
    while head <> nil do begin
      n := head;
      head := head^.next;
      Dispose(n);
    end;
  end;
  WriteLn(ok);
  WriteLn(MemAvail = before)
end.
