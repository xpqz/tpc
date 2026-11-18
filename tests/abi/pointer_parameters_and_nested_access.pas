program PtrNested;
type
  PNode = ^Node;
  Node = record v: Integer; next: PNode; end;
var head: PNode;
procedure Outer(n: Integer);
  procedure Push(v: Integer);
  var p: PNode;
  begin
    New(p);
    p^.v := v + n;
    p^.next := head;
    head := p;
  end;
var i: Integer;
begin
  for i := 1 to 5 do Push(i);
end;
function Sum(p: PNode): LongInt;
begin
  if p = nil then Sum := 0 else Sum := p^.v + Sum(p^.next);
end;
procedure FreeAll(var p: PNode);
var t: PNode;
begin
  while p <> nil do begin
    t := p;
    p := p^.next;
    Dispose(t);
  end;
end;
begin
  head := nil;
  Outer(100);
  Outer(1000);
  WriteLn(Sum(head));
  FreeAll(head);
  WriteLn(Sum(head), ' ', head = nil)
end.
