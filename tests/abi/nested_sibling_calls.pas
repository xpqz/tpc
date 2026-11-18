program NestedSiblingCalls;
{ sibling nested routines call each other and share the parent's locals }
procedure Parent;
var n: Integer;
  procedure Second; forward;
  procedure First;
  begin
    n := n + 1;
    WriteLn('first ', n);
    if n < 5 then Second
  end;
  procedure Second;
  begin
    n := n * 2;
    WriteLn('second ', n);
    First
  end;
begin
  n := 0;
  First;
  WriteLn('end ', n)
end;
begin
  Parent
end.
