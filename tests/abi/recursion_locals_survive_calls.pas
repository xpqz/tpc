program RecursionLocalsSurviveCalls;
{ each activation keeps its own locals across the recursive call, tree-shaped recursion }
var calls: Integer;
function Tree(depth, tag: Integer): LongInt;
var left, right, mine: LongInt;
begin
  calls := calls + 1;
  mine := tag;
  if depth = 0 then Tree := mine
  else begin
    left := Tree(depth - 1, tag * 2);
    right := Tree(depth - 1, tag * 2 + 1);
    Tree := left + right + mine
  end
end;
begin
  calls := 0;
  WriteLn(Tree(10, 1), ' ', calls);
  calls := 0;
  WriteLn(Tree(0, 5), ' ', calls)
end.
