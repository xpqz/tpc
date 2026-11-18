program NestedFunctionsInExpressions;
{ nested functions as operands and arguments in one expression: operand evaluation
  order matters because each call touches the outer local }
procedure Outer;
var tick: Integer;
  function Next: Integer;
  begin
    tick := tick + 1;
    Next := tick
  end;
  function Sub(a, b: Integer): Integer;
  begin
    Sub := a - b
  end;
begin
  tick := 0;
  WriteLn(Sub(Next, Next));
  WriteLn(Sub(Next * 10, Sub(Next, Next)));
  WriteLn(tick)
end;
begin
  Outer
end.
