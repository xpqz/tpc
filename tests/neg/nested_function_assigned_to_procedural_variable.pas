program NegTest;
{$F+}
type TFunc = function(x: Integer): Integer;
var f: TFunc;
function Outer(n: Integer): Integer;
  function Twice(k: Integer): Integer;
  begin
    Twice := k * 2
  end;
begin
  f := Twice;
  Outer := f(n)
end;
begin
  WriteLn(Outer(2))
end.
