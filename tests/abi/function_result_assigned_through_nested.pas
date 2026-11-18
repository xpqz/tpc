program FunctionResultAssignedThroughNested;
{ the enclosing function's result is assigned only in its own body; the nested function
  has its own result }
function Outer(n: Integer): Integer;
  function Inner(k: Integer): Integer;
  begin
    Inner := k * 10;
    Inner := k * 10 + 1
  end;
begin
  Outer := Inner(n);
  Outer := Inner(n + 1) + 5
end;
begin
  WriteLn(Outer(1));
  WriteLn(Outer(5))
end.
