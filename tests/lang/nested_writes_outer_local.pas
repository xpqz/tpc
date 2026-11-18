program NestedWritesOuterLocal;
{ the nested routine assigns to a local of the enclosing routine }
procedure Outer;
var x: Integer;
  procedure Put(v: Integer);
  begin
    x := v
  end;
begin
  x := 0;
  Put(42);
  WriteLn(x)
end;
begin
  Outer
end.
