program PointersAndPointerArrays;
type
  PInt = ^Integer;
  PNode = ^TNode;
  TNode = record
    value: Integer;
    next: PNode;
  end;
const
  none: PNode = nil;
  slots: array[1..3] of PInt = (nil, nil, nil);
  head: record
    first: PNode;
    count: Integer;
  end = (first: nil; count: 0);
var
  x, y: Integer;
begin
  x := 1;
  y := 2;
  slots[2] := @x;
  slots[3] := @y;
  WriteLn(none = nil, ' ', slots[1] = nil, ' ', slots[2]^ + slots[3]^);
  New(head.first);
  head.first^.value := 5;
  head.first^.next := none;
  head.count := head.count + 1;
  WriteLn(head.count, ' ', head.first^.value, ' ', head.first^.next = nil);
end.
