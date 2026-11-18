program SubrangeSizes;
type
  A = 0..255;
  B = -128..127;
  C = 0..256;
  D = -129..0;
  E = 0..65535;
  F = 0..65536;
  G = 100..200;
  H = 1000..1000;
begin
  WriteLn(SizeOf(A), ' ', SizeOf(B), ' ', SizeOf(C), ' ', SizeOf(D));
  WriteLn(SizeOf(E), ' ', SizeOf(F), ' ', SizeOf(G), ' ', SizeOf(H))
end.
