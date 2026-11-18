program IntTypingIntegerOverflowBeforeDiv;
var i, j: Integer;
begin
  i := 30000;
  j := 30000;
  WriteLn((i + j) div 2);
  WriteLn((i + j) mod 7);
  WriteLn(i + j > 0)
end.
