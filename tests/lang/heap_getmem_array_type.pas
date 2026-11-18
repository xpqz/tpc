program GetMemArray;
type
  TVec = array[0..999] of Integer;
  PVec = ^TVec;
var v: PVec; i, n: Integer; s: LongInt;
begin
  n := 200;
  GetMem(v, n * SizeOf(Integer));
  for i := 0 to n - 1 do v^[i] := i * 2;
  s := 0;
  for i := 0 to n - 1 do s := s + v^[i];
  WriteLn(s);
  FreeMem(v, n * SizeOf(Integer));
  WriteLn(SizeOf(TVec))
end.
