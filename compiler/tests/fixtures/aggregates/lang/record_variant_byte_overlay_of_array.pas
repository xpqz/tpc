program RecordVariantByteOverlayOfArray;
type Word2 = record case Integer of
       0: (w: Word);
       1: (bytes: array[0..1] of Byte);
       2: (i: Integer)
     end;
     Long4 = record case Boolean of
       True: (l: LongInt);
       False: (parts: array[1..2] of Word)
     end;
     Header = record
       tag: Byte;
       case Byte of
         0: (a, b: Byte);
         1: (both: Word);
         2: (x: Integer; y: Byte)
     end;
var w2: Word2; l4: Long4; h: Header;
begin
  w2.w := 258;
  WriteLn(w2.bytes[0], ' ', w2.bytes[1], ' ', w2.i);
  w2.bytes[0] := 255; w2.bytes[1] := 255;
  WriteLn(w2.w, ' ', w2.i);
  l4.l := 65537 + 2 * 65536;
  WriteLn(l4.parts[1], ' ', l4.parts[2]);
  l4.parts[2] := 1; l4.parts[1] := 0;
  WriteLn(l4.l);
  h.tag := 7; h.both := 513;
  WriteLn(h.tag, ' ', h.a, ' ', h.b);
  h.x := -2; h.y := 9;
  WriteLn(h.tag, ' ', h.x, ' ', h.y, ' ', h.both);
  WriteLn(SizeOf(Word2), ' ', SizeOf(Long4), ' ', SizeOf(Header))
end.
