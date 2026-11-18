program PtrArray;
type PI = ^Integer;
var a: array[1..5] of PI; i, sum: Integer;
begin
  for i := 1 to 5 do begin
    New(a[i]);
    a[i]^ := i * i;
  end;
  sum := 0;
  for i := 1 to 5 do sum := sum + a[i]^;
  WriteLn(sum);
  for i := 1 to 5 do WriteLn(a[i] = a[1], ' ', a[i]^);
  for i := 1 to 5 do Dispose(a[i])
end.
