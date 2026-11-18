program MemRelations;
var p: Pointer; before, after, mx: LongInt;
begin
  WriteLn(MemAvail > 0, ' ', MaxAvail > 0, ' ', MaxAvail <= MemAvail);
  before := MemAvail;
  GetMem(p, 1000);
  after := MemAvail;
  WriteLn(after < before, ' ', before - after >= 1000);
  WriteLn(MaxAvail <= MemAvail);
  FreeMem(p, 1000);
  WriteLn(MemAvail = before);
  mx := MaxAvail;
  GetMem(p, 5000);
  WriteLn(MaxAvail <= mx);
  FreeMem(p, 5000)
end.
