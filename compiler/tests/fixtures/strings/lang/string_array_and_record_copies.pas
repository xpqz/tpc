program StringArrayAndRecordCopies;
type Name = String[5]; Entry = record n: Name; tag: String[3]; w: Integer end;
var names: array[1..4] of Name; es: array[0..2] of Entry; e: Entry; long: String; i, j: Integer; tmp: Name;
begin
  long := 'abcdefghij';
  names[1] := long; names[2] := 'xy'; names[3] := names[1]; names[4] := names[2] + names[3];
  for i := 1 to 4 do WriteLn(i, ' ', names[i], ' ', Length(names[i]));
  e.n := long; e.tag := long; e.w := 7;
  es[0] := e; es[1] := es[0]; es[2].n := 'q'; es[2].tag := '';
  es[1].n[1] := 'Z'; es[0].tag[0] := Chr(1);
  WriteLn(es[0].n, es[0].tag, es[1].n, es[1].tag, es[2].n, Length(es[2].tag), es[1].w);
  for i := 1 to 3 do
    for j := 1 to 4 - i do
      if names[j] > names[j + 1] then
      begin
        tmp := names[j]; names[j] := names[j + 1]; names[j + 1] := tmp
      end;
  for i := 1 to 4 do Write(names[i], ';');
  WriteLn;
  names[2][0] := Chr(2);
  WriteLn(names[2], '|', names[2] + names[3] = Copy(names[2], 1, 2) + names[3])
end.
