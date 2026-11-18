program ProcVarVarParams;
{$F+}
{ procedural type with var parameters, called through a variable and a parameter }
type TMod = procedure(var x: Integer; by: Integer);
var m: TMod; n: Integer;
procedure AddTo(var x: Integer; by: Integer); begin x := x + by end;
procedure MulBy(var x: Integer; by: Integer); begin x := x * by end;
procedure Twice(p: TMod; var x: Integer; by: Integer);
begin
  p(x, by);
  p(x, by)
end;
begin
  n := 1;
  m := AddTo;
  m(n, 5);
  WriteLn(n);
  m := MulBy;
  m(n, 3);
  WriteLn(n);
  Twice(AddTo, n, 10);
  WriteLn(n);
  Twice(m, n, 2);
  WriteLn(n)
end.
