program NestedOuterLocalUsedBeforeAndAfterCall;
{ a local that lives in the frame (captured) keeps its value across calls and is also
  read in the outer body before the nested routine is ever called }
procedure Outer;
var captured, plain: Integer;
  procedure Touch;
  begin
    captured := captured + plain
  end;
begin
  captured := 1;
  plain := 2;
  WriteLn(captured, ' ', plain);
  Touch;
  WriteLn(captured, ' ', plain);
  plain := 40;
  Touch;
  WriteLn(captured, ' ', plain);
  captured := captured * 2 + plain;
  WriteLn(captured)
end;
begin
  Outer
end.
