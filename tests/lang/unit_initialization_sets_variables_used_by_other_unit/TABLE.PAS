unit Table;
interface
var
  Filled: Integer;
  Values: array[0..15] of Integer;
implementation
var
  i: Integer;
begin
  for i := 0 to 15 do
    Values[i] := i * i;
  Filled := 16;
end.
