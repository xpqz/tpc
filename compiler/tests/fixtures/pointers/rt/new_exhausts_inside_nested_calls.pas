program ExhaustNested;
type PB = ^Blk; Blk = record d: array[1..1000] of Byte; next: PB end;
var head: PB; n: Integer;
procedure Grab(depth: Integer);
var p: PB;
begin
  New(p);
  p^.next := head;
  head := p;
  n := n + 1;
  if depth > 0 then Grab(depth - 1) else Grab(5)
end;
begin
  head := nil;
  n := 0;
  WriteLn('start');
  Grab(3);
  WriteLn('not reached')
end.
