program ListReverse;
type
  PNode = ^Node;
  Node = record
    ch: Char;
    next: PNode;
  end;
var head, tail, n, prev, nxt: PNode; s: String; i: Integer;
procedure Show(p: PNode);
begin
  while p <> nil do begin
    Write(p^.ch);
    p := p^.next;
  end;
  WriteLn;
end;
begin
  head := nil;
  tail := nil;
  s := 'pointers';
  for i := 1 to Length(s) do begin
    New(n);
    n^.ch := s[i];
    n^.next := nil;
    if head = nil then head := n else tail^.next := n;
    tail := n;
  end;
  Show(head);
  prev := nil;
  n := head;
  while n <> nil do begin
    nxt := n^.next;
    n^.next := prev;
    prev := n;
    n := nxt;
  end;
  head := prev;
  Show(head);
  while head <> nil do begin
    n := head;
    head := head^.next;
    Dispose(n);
  end;
  WriteLn(head = nil)
end.
