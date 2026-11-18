program NestedEnclosingFunctionResultAssigned;
{ a nested routine may assign the result of an enclosing function: the assignment lands in
  the activation that is still running, and the last assignment wins }
function Pick(n: Integer): Integer;
  procedure Low;
  begin
    Pick := n - 100
  end;
  function Mid: Integer;
    procedure Deep;
    begin
      Pick := n * 1000
    end;
  begin
    if n > 5 then Deep;
    Mid := n + 1
  end;
var dummy: Integer;
begin
  Pick := 0;
  if n < 0 then Low;
  if n > 5 then dummy := Mid
  else if n > 0 then Pick := Mid
end;
function Count(n: Integer): Integer;
  procedure Step;
  begin
    if n > 0 then begin n := n - 1; Count := n * 10 end
  end;
begin
  Count := n;
  Step
end;
begin
  WriteLn(Pick(-7), ' ', Pick(0), ' ', Pick(3), ' ', Pick(9));
  WriteLn(Count(5))
end.
