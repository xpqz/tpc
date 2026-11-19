program MutationInLoops;
const
  table: array[1..5] of Integer = (5, 4, 3, 2, 1);
  total: LongInt = 0;
  name: string[8] = 'abc';
var
  i, j, t: Integer;
begin
  for i := 1 to 4 do
    for j := 1 to 5 - i do
      if table[j] > table[j + 1] then
      begin
        t := table[j];
        table[j] := table[j + 1];
        table[j + 1] := t;
      end;
  for i := 1 to 5 do
  begin
    total := total + table[i] * 1000;
    name := name + Chr(Ord('0') + table[i]);
  end;
  WriteLn(table[1], table[2], table[3], table[4], table[5], ' ', total, ' ', name);
end.
