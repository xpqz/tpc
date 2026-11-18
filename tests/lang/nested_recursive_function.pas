program NestedRecursiveFunction;
{ a nested function calling itself }
function Outer(n: Integer): LongInt;
  function Fact(k: Integer): LongInt;
  begin
    if k <= 1 then Fact := 1 else Fact := k * Fact(k - 1)
  end;
begin
  Outer := Fact(n)
end;
begin
  WriteLn(Outer(5), ' ', Outer(10))
end.
