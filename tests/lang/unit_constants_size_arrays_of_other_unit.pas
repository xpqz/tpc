program UnitConstantsSizeArraysOfOtherUnit;
uses Sizes;
var
  a: array[1..Count] of Integer;
  i: Integer;
begin
  for i := 1 to Count do
    a[i] := i * Step;
  WriteLn(a[1], ' ', a[Count]);
  WriteLn(SizeOf(a));
  WriteLn(Title + Suffix);
end.
