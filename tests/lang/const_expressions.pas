program ConstExpressions;
const
  X = (3 + 4) * 5 - 2;
  M = 17 mod 5;
  D = 17 div 5;
  Sh = 1 shl 10;
  Sr = 1024 shr 3;
  Bw = $F0 or $0F;
  Bx = $FF xor $0F;
  Ba = $FF and $3C;
  Neg = -(3 + 4);
  Y = X * 2;
begin
  WriteLn(X, ' ', M, ' ', D);
  WriteLn(Sh, ' ', Sr);
  WriteLn(Bw, ' ', Bx, ' ', Ba);
  WriteLn(Neg, ' ', Y)
end.
