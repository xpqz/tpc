program BitwiseOperatorsInteger;
var
  i, j: Integer;
  l: LongInt;
begin
  i := 12;
  j := 10;
  WriteLn(i and j, ' ', i or j, ' ', i xor j);
  i := not i;
  WriteLn(i);
  i := -1;
  j := $0F0F;
  WriteLn(i and j);
  WriteLn(i xor j);
  i := 0;
  i := not i;
  WriteLn(i);
  l := $0F0F0F0F;
  WriteLn(l and $00FF00FF, ' ', l or $70000000, ' ', l xor $0F0F0F0F);
  l := not l;
  WriteLn(l)
end.
