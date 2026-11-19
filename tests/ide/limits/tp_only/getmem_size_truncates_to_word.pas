program Probe;
var Q: Pointer; N, Before: LongInt;
begin
  N := 70000;
  Before := MemAvail;
  GetMem(Q, N);
  WriteLn('asked 70000, taken ', Before - MemAvail);
end.
