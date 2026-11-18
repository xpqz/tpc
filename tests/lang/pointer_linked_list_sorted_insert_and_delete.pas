program ListSorted;
type
  PNode = ^Node;
  Node = record
    key: Integer;
    next: PNode;
  end;
var head: PNode;
procedure Insert(k: Integer);
var n, p: PNode;
begin
  New(n);
  n^.key := k;
  if (head = nil) or (k < head^.key) then begin
    n^.next := head;
    head := n;
  end else begin
    p := head;
    while (p^.next <> nil) and (p^.next^.key < k) do p := p^.next;
    n^.next := p^.next;
    p^.next := n;
  end;
end;
procedure Delete(k: Integer);
var p, d: PNode;
begin
  if head = nil then Exit;
  if head^.key = k then begin
    d := head;
    head := head^.next;
    Dispose(d);
    Exit;
  end;
  p := head;
  while (p^.next <> nil) and (p^.next^.key <> k) do p := p^.next;
  if p^.next <> nil then begin
    d := p^.next;
    p^.next := d^.next;
    Dispose(d);
  end;
end;
procedure Show;
var p: PNode;
begin
  p := head;
  while p <> nil do begin
    Write(p^.key, ' ');
    p := p^.next;
  end;
  WriteLn;
end;
var i: Integer;
begin
  head := nil;
  for i := 1 to 10 do Insert((i * 7) mod 11);
  Show;
  Delete(0);
  Delete(10);
  Delete(5);
  Delete(99);
  Show;
  Insert(5);
  Insert(5);
  Show
end.
