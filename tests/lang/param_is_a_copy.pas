program ParamIsACopy;
var x: Integer;
procedure Change(n: Integer);
begin
  n := n + 100;
  WriteLn('inside ', n)
end;
begin
  x := 5;
  Change(x);
  WriteLn('outside ', x);
  Change(x + 1);
  Change(7);
  WriteLn(x)
end.
