program TypedconstPointerNil;
type
  PInt = ^Integer;
const
  p: PInt = nil;
  q: Pointer = nil;
var
  x: Integer;
begin
  WriteLn(p = nil, ' ', q = nil);
  x := 5;
  p := @x;
  WriteLn(p <> nil, ' ', p^);
end.
