program UnaryMinusAndPlus;
var
  i: Integer;
  l: LongInt;
begin
  i := 5;
  WriteLn(-i);
  WriteLn(+i);
  WriteLn(-(-i));
  i := -32767;
  WriteLn(-i);
  l := 2147483647;
  WriteLn(-l);
  l := -l;
  WriteLn(l - 1)
end.
