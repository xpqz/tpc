program ArraySwap;
type V = array[1..4] of Integer;
var a, b, t: V; i: Integer;
begin
  for i := 1 to 4 do begin a[i] := i; b[i] := 10 * i end;
  a := a;
  t := a; a := b; b := t;
  for i := 1 to 4 do Write(a[i], ' ', b[i], ' ');
  WriteLn
end.
