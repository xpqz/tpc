program HeapDeep;
function Build(n: Integer): LongInt;
var p: ^LongInt;
begin
  if n = 0 then Build := 0
  else begin
    GetMem(p, SizeOf(LongInt));
    p^ := n;
    Build := p^ + Build(n - 1);
    FreeMem(p, SizeOf(LongInt));
  end;
end;
var before: LongInt;
begin
  before := MemAvail;
  WriteLn(Build(300));
  WriteLn(MemAvail = before)
end.
