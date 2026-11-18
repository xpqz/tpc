program ConstScalars;
const
  A = 10;
  B = A * 2 + 1;
  Neg = -5;
  Big = 100000;
  Hex = $FF;
  Ch = 'x';
  Flag = True;
  Off = False;
var n: Integer;
begin
  WriteLn(A, ' ', B, ' ', Neg, ' ', Big, ' ', Hex);
  WriteLn(Ch, ' ', Flag, ' ', Off);
  n := B - A;
  WriteLn(n);
  WriteLn(A + B);
  WriteLn(Big div A)
end.
