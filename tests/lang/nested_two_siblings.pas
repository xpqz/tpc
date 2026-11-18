program NestedTwoSiblings;
{ two nested routines in one parent, one calling the other }
procedure Parent;
var n: Integer;
  procedure AddOne;
  begin
    n := n + 1
  end;
  procedure AddThree;
  begin
    AddOne; AddOne; AddOne
  end;
begin
  n := 0;
  AddThree;
  AddOne;
  WriteLn(n)
end;
begin
  Parent
end.
