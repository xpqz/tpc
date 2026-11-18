program MarkProc;
type PNode = ^Node; Node = record v: Integer; next: PNode; end;
function Build(n: Integer): LongInt;
var m: Pointer; head, p: PNode; i: Integer; s: LongInt;
begin
  Mark(m);
  head := nil;
  for i := 1 to n do begin
    New(p);
    p^.v := i;
    p^.next := head;
    head := p;
  end;
  s := 0;
  p := head;
  while p <> nil do begin
    s := s + p^.v;
    p := p^.next;
  end;
  Release(m);
  Build := s;
end;
var before: LongInt; i: Integer;
begin
  before := MemAvail;
  for i := 1 to 50 do
    if Build(i * 10) <> LongInt(i * 10) * (i * 10 + 1) div 2 then WriteLn('wrong ', i);
  WriteLn(Build(100));
  WriteLn(MemAvail = before)
end.
