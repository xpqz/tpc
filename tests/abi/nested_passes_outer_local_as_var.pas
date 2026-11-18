program NestedPassesOuterLocalAsVar;
{ a nested routine passes a captured outer local as a var argument }
procedure Add(var target: Integer; amount: Integer);
begin
  target := target + amount
end;
procedure Outer;
var total: Integer;
  procedure Feed(n: Integer);
  begin
    Add(total, n);
    Add(total, n)
  end;
begin
  total := 0;
  Feed(3);
  Feed(4);
  Add(total, 100);
  WriteLn(total)
end;
begin
  Outer
end.
