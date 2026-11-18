program VarParamNestedRoutineSeesIt;
{ a nested routine reads and writes the enclosing routine's var parameter, so the
  caller's variable changes through the static link and the stored address }
procedure Outer(var acc: Integer);
  procedure Add(n: Integer);
  begin
    acc := acc + n
  end;
  procedure AddTwice(n: Integer);
  begin
    Add(n);
    Add(n)
  end;
begin
  Add(1);
  AddTwice(10);
  acc := acc * 2
end;
var total: Integer;
begin
  total := 5;
  Outer(total);
  WriteLn(total);
  Outer(total);
  WriteLn(total)
end.
