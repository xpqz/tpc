program VarParamAliasSameVariable;
{ the same variable passed for two var parameters: writes through one are seen through
  the other }
procedure Twice(var a, b: Integer);
begin
  a := 1;
  b := b + 10;
  a := a + b;
  WriteLn(a, ' ', b)
end;
procedure Mix(var a: Integer; v: Integer; var b: Integer);
begin
  a := a + 1;
  WriteLn(v, ' ', a, ' ', b);
  b := b * 2;
  WriteLn(v, ' ', a, ' ', b)
end;
var x, y: Integer;
begin
  x := 0; y := 5;
  Twice(x, x);
  WriteLn(x);
  Twice(x, y);
  WriteLn(x, ' ', y);
  x := 3;
  Mix(x, x, x);
  WriteLn(x)
end.
