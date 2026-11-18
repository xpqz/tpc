program NestedFunctionBasic;
{ a function declared inside a function }
function Outer(n: Integer): Integer;
  function Sq(k: Integer): Integer;
  begin
    Sq := k * k
  end;
begin
  Outer := Sq(n) + Sq(n + 1)
end;
begin
  WriteLn(Outer(2));
  WriteLn(Outer(10))
end.
