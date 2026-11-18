program PtrForward;
type
  PNode = ^Node;
  Node = record
    value: Integer;
    next: PNode;
  end;
  PA = ^TB;
  TB = record
    back: PA;
    n: Byte;
  end;
var a, b: PNode; x, y: PA;
begin
  New(a);
  New(b);
  a^.value := 1;
  a^.next := b;
  b^.value := 2;
  b^.next := nil;
  WriteLn(a^.value, ' ', a^.next^.value, ' ', a^.next^.next = nil);
  New(x);
  New(y);
  x^.back := nil;
  x^.n := 1;
  y^.back := x;
  y^.n := 2;
  WriteLn(y^.back^.n, ' ', y^.back^.back = nil);
  Dispose(y);
  Dispose(x);
  Dispose(b);
  Dispose(a)
end.
