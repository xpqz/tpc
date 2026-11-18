program TreeStats;
type
  PTree = ^Tree;
  Tree = record
    key: LongInt;
    left, right: PTree;
  end;
var root: PTree;
procedure Insert(var t: PTree; k: LongInt);
begin
  if t = nil then begin
    New(t);
    t^.key := k;
    t^.left := nil;
    t^.right := nil;
  end else if k < t^.key then Insert(t^.left, k)
  else if k > t^.key then Insert(t^.right, k);
end;
function Height(t: PTree): Integer;
var l, r: Integer;
begin
  if t = nil then Height := 0
  else begin
    l := Height(t^.left);
    r := Height(t^.right);
    if l > r then Height := l + 1 else Height := r + 1;
  end;
end;
function Count(t: PTree): Integer;
begin
  if t = nil then Count := 0 else Count := 1 + Count(t^.left) + Count(t^.right);
end;
function Find(t: PTree; k: LongInt): Boolean;
begin
  if t = nil then Find := False
  else if k = t^.key then Find := True
  else if k < t^.key then Find := Find(t^.left, k)
  else Find := Find(t^.right, k);
end;
function Min(t: PTree): LongInt;
begin
  while t^.left <> nil do t := t^.left;
  Min := t^.key;
end;
function Max(t: PTree): LongInt;
begin
  while t^.right <> nil do t := t^.right;
  Max := t^.key;
end;
var i: Integer; x: LongInt;
begin
  root := nil;
  x := 12345;
  for i := 1 to 200 do begin
    x := (x * 1103 + 12347) mod 32749;
    Insert(root, x);
  end;
  WriteLn(Count(root) <= 200, ' ', Count(root) > 100);
  WriteLn(Height(root) < Count(root), ' ', Height(root) >= 8);
  WriteLn(Find(root, Min(root)), ' ', Find(root, Max(root)), ' ', Find(root, -1));
  WriteLn(Min(root) <= Max(root));
  Insert(root, 100000);
  Insert(root, -100000);
  WriteLn(Min(root), ' ', Max(root))
end.
