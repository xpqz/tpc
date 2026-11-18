program ListStack;
type
  PNode = ^Node;
  Node = record
    value: Integer;
    next: PNode;
  end;
var top: PNode;
procedure Push(v: Integer);
var n: PNode;
begin
  New(n);
  n^.value := v;
  n^.next := top;
  top := n;
end;
function Pop: Integer;
var n: PNode;
begin
  n := top;
  Pop := n^.value;
  top := n^.next;
  Dispose(n);
end;
var i, sum: Integer;
begin
  top := nil;
  for i := 1 to 10 do Push(i * i);
  sum := 0;
  while top <> nil do begin
    Write(Pop, ' ');
    sum := sum + 1;
  end;
  WriteLn;
  WriteLn(sum, ' ', top = nil)
end.
