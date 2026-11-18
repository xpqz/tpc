program NestedReadsOuterLocal;
{ the nested routine reads a local of the enclosing routine as it is at call time }
procedure Outer;
var x: Integer;
  procedure Show;
  begin
    WriteLn(x)
  end;
begin
  x := 1;
  Show;
  x := 2;
  Show
end;
begin
  Outer
end.
