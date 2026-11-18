program MarkRelease;
type PI = ^Integer;
var m, m2: Pointer; p: PI; i, round: Integer; base, after: LongInt; same: Boolean;
begin
  Mark(m);
  base := MemAvail;
  same := true;
  for round := 1 to 20 do
  begin
    Mark(m2);
    for i := 1 to 50 do begin New(p); p^ := i end;
    if MemAvail >= base then same := false;
    Release(m2);
    if MemAvail <> base then same := false
  end;
  WriteLn(same);
  for i := 1 to 10 do begin New(p); Dispose(p) end;
  Release(m);
  WriteLn(MemAvail = base);
  Mark(m);
  GetMem(p, 100);
  Mark(m2);
  GetMem(p, 100);
  after := MemAvail;
  Release(m2);
  WriteLn(MemAvail > after, ' ', MemAvail < base);
  Release(m);
  WriteLn(MemAvail = base, ' ', MaxAvail <= MemAvail)
end.
