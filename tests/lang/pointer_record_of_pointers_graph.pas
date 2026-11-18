program PtrGraph;
type
  PNode = ^Node;
  Node = record
    id: Integer;
    a, b: PNode;
  end;
var n1, n2, n3: PNode;
begin
  New(n1); New(n2); New(n3);
  n1^.id := 1; n2^.id := 2; n3^.id := 3;
  n1^.a := n2; n1^.b := n3;
  n2^.a := n3; n2^.b := n1;
  n3^.a := n1; n3^.b := nil;
  WriteLn(n1^.a^.a^.id, n1^.b^.a^.id, n2^.b^.b^.id);
  WriteLn(n3^.b = nil, ' ', n1^.a^.b^.a^.a^.id);
  n1^.a^.a^.id := 30;
  WriteLn(n3^.id);
  Dispose(n3); Dispose(n2); Dispose(n1)
end.
