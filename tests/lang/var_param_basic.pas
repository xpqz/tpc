program VarParamBasic;
{ a var parameter changes the caller's variable; a value parameter does not }
procedure Set7(var x: Integer);
begin
  x := 7
end;
procedure Try7(x: Integer);
begin
  x := 7
end;
var a, b: Integer;
begin
  a := 1; b := 1;
  Set7(a);
  Try7(b);
  WriteLn(a, ' ', b)
end.
