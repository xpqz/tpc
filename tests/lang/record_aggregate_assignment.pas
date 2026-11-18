program RecordAssign;
type R = record a: Integer; s: String[10]; c: array[1..3] of Byte end;
var x, y: R; i: Integer;
begin
  x.a := 7; x.s := 'hello';
  for i := 1 to 3 do x.c[i] := i * 11;
  y := x;
  x.s := 'bye'; x.c[2] := 0; x.a := 0;
  WriteLn(y.a, ' ', y.s, ' ', y.c[1], y.c[2], y.c[3]);
  WriteLn(x.a, ' ', x.s, ' ', x.c[1], x.c[2], x.c[3]);
  x := y; y.s := 'zzz';
  WriteLn(x.s, ' ', y.s)
end.
