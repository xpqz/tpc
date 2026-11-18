program VariantNested;
type
  R = packed record
    case Boolean of
      True: (x: Byte; y: Byte);
      False: (z: Integer)
  end;
  ST = packed record
    head: Byte;
    case Byte of
      0: (r: R; t: Byte);
      1: (w: Word; q: Byte);
      2: ()
  end;
var s: ST;
begin
  WriteLn(SizeOf(R), ' ', SizeOf(ST));
  s.head := 1; s.r.x := 2; s.r.y := 3; s.t := 4;
  WriteLn(s.r.z, ' ', s.w, ' ', s.q);
  s.w := $0102;
  WriteLn(s.r.x, ' ', s.r.y)
end.
