program ArrayOfRecords;
type ET = record name: String[8]; age: Integer end;
var t: array[1..4] of ET; i, sum: Integer; e: ET;
begin
  t[1].name := 'ann'; t[1].age := 31;
  t[2].name := 'bob'; t[2].age := 27;
  t[3].name := 'cy'; t[3].age := 45;
  t[4].name := 'dee'; t[4].age := 19;
  sum := 0;
  for i := 1 to 4 do sum := sum + t[i].age;
  WriteLn(sum);
  e := t[2]; t[2] := t[3]; t[3] := e;
  for i := 1 to 4 do WriteLn(t[i].name, ' ', t[i].age)
end.
