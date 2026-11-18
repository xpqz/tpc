program VariantLong;
type L = packed record case Integer of 0: (l: LongInt); 1: (lo, hi: Word); 2: (b: array[0..3] of Byte) end;
var v: L;
begin
  v.l := $12345678;
  WriteLn(v.lo, ' ', v.hi, ' ', v.b[0], ' ', v.b[1], ' ', v.b[2], ' ', v.b[3]);
  v.hi := 0; v.b[0] := 1;
  WriteLn(v.l);
  v.l := -1;
  WriteLn(v.lo, ' ', v.hi, ' ', v.b[3]);
  WriteLn(SizeOf(L))
end.
