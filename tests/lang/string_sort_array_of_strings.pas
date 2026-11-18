program SortStrings;
var a: array[1..6] of String[10]; i, j: Integer; t: String[10];
begin
  a[1] := 'pear'; a[2] := 'apple'; a[3] := 'fig'; a[4] := 'Banana'; a[5] := 'apple2'; a[6] := '';
  for i := 1 to 5 do
    for j := 1 to 6 - i do
      if a[j] > a[j + 1] then begin t := a[j]; a[j] := a[j + 1]; a[j + 1] := t end;
  for i := 1 to 6 do WriteLn('[', a[i], ']')
end.
