program HeapFrames;
type PArr = ^TArr; TArr = array[1..50] of LongInt;
function Fill(n: Integer): LongInt;
var m: Pointer; p: PArr; i: Integer; s: LongInt; buf: array[1..20] of Integer;
begin
  Mark(m);
  New(p);
  for i := 1 to 50 do p^[i] := i * n;
  for i := 1 to 20 do buf[i] := i;
  s := 0;
  for i := 1 to 50 do s := s + p^[i];
  for i := 1 to 20 do s := s + buf[i];
  if n > 1 then s := s + Fill(n - 1);
  Release(m);
  Fill := s;
end;
var before: LongInt;
begin
  before := MemAvail;
  WriteLn(Fill(1), ' ', Fill(5));
  WriteLn(MemAvail = before)
end.
