program GetMemSizes;
type PB = ^Byte; PBA = ^BA; BA = array[0..9999] of Byte; PI = ^Integer;
var p, q: PBA; n, i: Integer; ok: Boolean; r: PI; s: Pointer;
begin
  for n := 1 to 5 do
  begin
    GetMem(p, n * 100);
    for i := 0 to n * 100 - 1 do p^[i] := (i + n) mod 256;
    ok := true;
    for i := 0 to n * 100 - 1 do
      if p^[i] <> (i + n) mod 256 then ok := false;
    WriteLn(n, ' ', ok);
    FreeMem(p, n * 100)
  end;
  GetMem(p, 1000); GetMem(q, 1000);
  for i := 0 to 999 do begin p^[i] := 1; q^[i] := 2 end;
  ok := true;
  for i := 0 to 999 do
    if (p^[i] <> 1) or (q^[i] <> 2) then ok := false;
  WriteLn(ok);
  FreeMem(q, 1000); FreeMem(p, 1000);
  GetMem(r, SizeOf(Integer));
  r^ := -5;
  WriteLn(r^);
  FreeMem(r, SizeOf(Integer));
  GetMem(s, 0);
  WriteLn(s <> nil)
end.
