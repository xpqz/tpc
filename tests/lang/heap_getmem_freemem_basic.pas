program GetMemBasic;
type PB = ^Byte;
var p: PB; q: ^Integer; i: Integer; sum: LongInt; b: PB;
begin
  GetMem(p, 100);
  for i := 0 to 99 do begin
    b := PB(LongInt(p) + i);
    b^ := i;
  end;
  sum := 0;
  for i := 0 to 99 do begin
    b := PB(LongInt(p) + i);
    sum := sum + b^;
  end;
  WriteLn(sum);
  FreeMem(p, 100);
  GetMem(q, SizeOf(Integer));
  q^ := -123;
  WriteLn(q^);
  FreeMem(q, SizeOf(Integer))
end.
