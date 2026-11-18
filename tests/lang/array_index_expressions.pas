program ArrayIndexExpr;
var a: array[0..9] of Integer; i, j: Integer;
function Next(x: Integer): Integer; begin Next := x + 1 end;
begin
  for i := 0 to 9 do a[i] := 9 - i;
  i := 2; j := 3;
  WriteLn(a[i + j], ' ', a[i * j], ' ', a[a[a[1]]], ' ', a[Next(i)], ' ', a[9 - a[9]], ' ', a[(i + j) mod 4]);
  a[a[0]] := 77;
  a[Next(4)] := a[Next(5)] + 1000;
  for i := 0 to 9 do Write(a[i], ' ');
  WriteLn
end.
