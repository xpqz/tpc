program ArrayPacked;
type P7 = packed array[1..7] of Integer; P9 = packed array[1..9] of Boolean;
  PT = packed array[1..5] of Byte; UT = array[1..5] of Byte;
var p: PT; u: UT; i: Integer;
begin
  for i := 1 to 5 do p[i] := i * 50;
  for i := 1 to 5 do u[i] := p[i];
  for i := 1 to 5 do Write(u[i], ' ');
  WriteLn;
  WriteLn(SizeOf(PT), ' ', SizeOf(UT), ' ', SizeOf(P7), ' ', SizeOf(P9))
end.
