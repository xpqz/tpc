program TreeChain;
type
  PTree = ^Tree;
  Tree = record
    key: Integer;
    left, right: PTree;
  end;
var root, p: PTree; i: Integer;
procedure Insert(var t: PTree; k: Integer);
begin
  if t = nil then begin
    New(t);
    t^.key := k;
    t^.left := nil;
    t^.right := nil;
  end else if k < t^.key then Insert(t^.left, k)
  else Insert(t^.right, k);
end;
begin
  root := nil;
  for i := 1 to 100 do Insert(root, i);
  p := root;
  i := 0;
  while p <> nil do begin
    i := i + 1;
    if p^.left <> nil then WriteLn('left child');
    p := p^.right;
  end;
  WriteLn(i)
end.
