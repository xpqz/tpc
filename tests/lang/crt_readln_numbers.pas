program ReadLnNumbers;
uses Crt;
var
  A, B: Integer;
  L: LongInt;
  R: Real;
begin
  ClrScr;
  Write('a b? ');
  ReadLn(A, B);
  WriteLn(A + B);
  Write('long? ');
  ReadLn(L);
  WriteLn(L * 2);
  Write('real? ');
  ReadLn(R);
  WriteLn(R * 2:0:2);
  Write('neg? ');
  ReadLn(A);
  WriteLn(A);
end.
