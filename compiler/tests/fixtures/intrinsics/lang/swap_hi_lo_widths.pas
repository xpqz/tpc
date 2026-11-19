program SwapHiLo;
var i: Integer; w: Word; l: LongInt; b: Byte; sh: ShortInt;
begin
  w := 4660;
  WriteLn(Swap(w), ' ', Hi(w), ' ', Lo(w));
  i := 258;
  WriteLn(Swap(i), ' ', Hi(i), ' ', Lo(i));
  i := 255;
  WriteLn(Swap(i));
  i := -2;
  WriteLn(Swap(i), ' ', Hi(i), ' ', Lo(i));
  l := 70000;
  WriteLn(Swap(l), ' ', Hi(l), ' ', Lo(l));
  l := -70000;
  WriteLn(Swap(l), ' ', Hi(l), ' ', Lo(l));
  l := 65536;
  WriteLn(Swap(l), ' ', Hi(l), ' ', Lo(l));
  b := 200;
  WriteLn(Swap(b), ' ', Hi(b), ' ', Lo(b));
  sh := -1;
  WriteLn(Swap(sh), ' ', Hi(sh), ' ', Lo(sh));
  WriteLn(Swap(255), ' ', Hi(4660), ' ', Lo(4660));
  w := Swap(Swap(w));
  WriteLn(w);
  w := Swap(w) + 1;
  WriteLn(w);
  WriteLn(Hi(w) + Lo(w))
end.
