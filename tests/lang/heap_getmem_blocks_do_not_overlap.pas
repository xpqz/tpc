program GetMemDisjoint;
type PB = ^Byte;
var p: array[1..3] of PB; i, j: Integer; ok: Boolean; b: PB;
begin
  for i := 1 to 3 do begin
    GetMem(p[i], 10);
    for j := 0 to 9 do begin
      b := PB(LongInt(p[i]) + j);
      b^ := i * 16 + j;
    end;
  end;
  ok := True;
  for i := 1 to 3 do
    for j := 0 to 9 do begin
      b := PB(LongInt(p[i]) + j);
      if b^ <> i * 16 + j then ok := False;
    end;
  WriteLn(ok);
  for i := 3 downto 1 do FreeMem(p[i], 10)
end.
