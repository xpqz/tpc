program RealVarParam;
procedure Swap(var a, b: Real);
var t: Real;
begin
  t := a; a := b; b := t;
end;
procedure Stats(var lo, hi, avg: Real; n: Integer);
var i: Integer; x, sum: Real;
begin
  lo := 1E6; hi := -1E6; sum := 0;
  for i := 1 to n do begin
    x := i * 0.5 - 2;
    if x < lo then lo := x;
    if x > hi then hi := x;
    sum := sum + x;
  end;
  avg := sum / n;
end;
var a, b, c: Real; arr: array[1..3] of Real;
begin
  a := 1.5; b := -2.25;
  Swap(a, b);
  WriteLn(a:0:2, ' ', b:0:2);
  Stats(a, b, c, 8);
  WriteLn(a:0:2, ' ', b:0:2, ' ', c:0:3);
  arr[1] := 1; arr[2] := 2; arr[3] := 3;
  Swap(arr[1], arr[3]);
  WriteLn(arr[1]:0:0, arr[2]:0:0, arr[3]:0:0)
end.
