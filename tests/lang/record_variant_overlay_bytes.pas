program VariantBytes;
type
  VT = packed record
    case Boolean of
      False: (w: Word);
      True: (lo, hi: Byte)
  end;
var v: VT;
begin
  WriteLn(SizeOf(VT));
  v.w := $1234;
  WriteLn(v.lo, ' ', v.hi);
  v.lo := 255; v.hi := 1;
  WriteLn(v.w);
  v.w := 0;
  WriteLn(v.lo + v.hi)
end.
