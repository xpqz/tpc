program NestedFunctionResultsUseOuter;
{ nested functions return values computed from outer locals and parameters }
function Outer(base: Integer): Integer;
var scale, r: Integer;
  function Scaled(k: Integer): Integer;
  begin
    Scaled := (base + k) * scale
  end;
  function Twice(k: Integer): Integer;
  begin
    Twice := Scaled(k) + Scaled(k + 1)
  end;
begin
  scale := 3;
  r := Twice(1);
  scale := 10;
  Outer := r + Twice(2)
end;
begin
  WriteLn(Outer(0));
  WriteLn(Outer(5))
end.
