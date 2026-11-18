program TreeInorder;
type
  PTree = ^Tree;
  Tree = record
    key: Integer;
    left, right: PTree;
  end;
var root: PTree;
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
procedure InOrder(t: PTree);
begin
  if t <> nil then begin
    InOrder(t^.left);
    Write(t^.key, ' ');
    InOrder(t^.right);
  end;
end;
procedure PreOrder(t: PTree);
begin
  if t <> nil then begin
    Write(t^.key, ' ');
    PreOrder(t^.left);
    PreOrder(t^.right);
  end;
end;
procedure Free(var t: PTree);
begin
  if t <> nil then begin
    Free(t^.left);
    Free(t^.right);
    Dispose(t);
    t := nil;
  end;
end;
var i: Integer;
begin
  root := nil;
  for i := 1 to 15 do Insert(root, (i * 7) mod 16);
  InOrder(root);
  WriteLn;
  PreOrder(root);
  WriteLn;
  Free(root);
  WriteLn(root = nil)
end.
