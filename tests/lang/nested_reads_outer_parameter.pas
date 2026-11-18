program NestedReadsOuterParameter;
{ the nested routine uses the enclosing routine's value parameter and a global }
var scale: Integer;
function Outer(base: Integer): Integer;
  function Scaled(k: Integer): Integer;
  begin
    Scaled := (base + k) * scale
  end;
begin
  Outer := Scaled(1) + Scaled(2)
end;
begin
  scale := 10;
  WriteLn(Outer(5));
  scale := 2;
  WriteLn(Outer(5))
end.
