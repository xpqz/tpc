program ArrayBoolIndex;
var a: array[Boolean] of Integer; b: Boolean;
begin
  a[False] := 10; a[True] := 20;
  for b := False to True do WriteLn(b, ' ', a[b]);
  WriteLn(a[3 > 2], ' ', a[3 < 2], ' ', SizeOf(a));
  a[not b] := 99;
  WriteLn(a[False], ' ', a[True])
end.
