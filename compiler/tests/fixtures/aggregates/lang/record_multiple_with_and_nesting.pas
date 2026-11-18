program RecordMultipleWithAndNesting;
type Inner = record n: Integer; flag: Boolean end;
     Outer = record id: Integer; in1: Inner; list: array[1..2] of Inner end;
var a, b: Outer; k: Integer;
function Next: Integer;
begin
  k := k + 1;
  Next := k
end;
begin
  with a do
  begin
    id := 1;
    in1.n := 10; in1.flag := True;
    with in1 do n := n + id;
    with list[1] do begin n := 100; flag := False end
  end;
  with a, a.in1 do WriteLn(id, ' ', n, ' ', flag);
  b := a;
  with b do begin id := 2; with in1, list[1] do n := n + 5 end;
  WriteLn(a.in1.n, ' ', b.in1.n, ' ', a.list[1].n, ' ', b.list[1].n);
  k := 0;
  with a.list[Next] do n := 7;
  with a.list[Next] do n := 8;
  WriteLn(k, ' ', a.list[1].n, ' ', a.list[2].n)
end.
