program TypedconstCharArraysFromStringAndList;
const
  tag: array[1..4] of Char = 'ABCD';
  mixed: array[1..2] of Char = ('x', 'y');
var
  i: Integer;
begin
  for i := 1 to 4 do
    Write(tag[i]);
  WriteLn;
  WriteLn(mixed[1], mixed[2]);
end.
