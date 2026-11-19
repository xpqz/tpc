program NegTest;
type Rec = record Size: Integer end;
var r: Rec;
function Size(i: Integer): Integer;
begin
  Size := i
end;
begin
  with r do
    WriteLn(Size(3))
end.
