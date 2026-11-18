program IntTypingMixedExpressionChain;
var
  b: Byte;
  i: Integer;
  l: LongInt;
begin
  b := 200;
  i := 200;
  l := 200;
  WriteLn(b + b + b);
  WriteLn(i * i * i);
  WriteLn(l * l * l);
  WriteLn(b * b + l);
  WriteLn(b * b div 7)
end.
