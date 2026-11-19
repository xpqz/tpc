program ScalarsEdgeValues;
type
  TColor = (Red, Green, Blue);
  TLow = -5..5;
  TBig = 1000..2000;
const
  lowest: Integer = -32768;
  top: Integer = 32767;
  wmax: Word = 65535;
  lmin: LongInt = -2147483647;
  smin: ShortInt = -128;
  bmax: Byte = 255;
  neg: TLow = -5;
  big: TBig = 2000;
  tc: TColor = Green;
  yes: Boolean = True;
  no: Boolean = False;
  c0: Char = #0;
  quote: Char = '''';
  pi: Real = 3.25;
  whole: Real = 7;
  negr: Real = -1.5;
begin
  WriteLn(lowest, ' ', top, ' ', wmax, ' ', lmin, ' ', smin, ' ', bmax);
  WriteLn(neg, ' ', big, ' ', Ord(tc), ' ', yes, ' ', no, ' ', Ord(c0), ' ', quote);
  WriteLn(pi:0:2, ' ', whole:0:1, ' ', negr:0:1);
  lowest := lowest + 1;
  bmax := bmax - 5;
  tc := Succ(tc);
  pi := pi * 2;
  WriteLn(lowest, ' ', bmax, ' ', Ord(tc), ' ', pi:0:2);
end.
