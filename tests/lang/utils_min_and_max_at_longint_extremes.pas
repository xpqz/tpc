program UtilsMinMax;
uses Utils;
const
  Lowest = -MaxLongInt - 1;
begin
  WriteLn(Min(1, 2), ' ', Min(2, 1), ' ', Min(5, 5), ' ', Min(-1, 1), ' ', Min(-7, -3));
  WriteLn(Max(1, 2), ' ', Max(2, 1), ' ', Max(5, 5), ' ', Max(-1, 1), ' ', Max(-7, -3));
  WriteLn(Min(Lowest, MaxLongInt), ' ', Max(Lowest, MaxLongInt));
  WriteLn(Min(Lowest, 0), ' ', Max(Lowest, 0), ' ', Min(MaxLongInt, 0), ' ', Max(MaxLongInt, 0));
  WriteLn(Min(Lowest, Lowest), ' ', Max(MaxLongInt, MaxLongInt));
  WriteLn(Min(32767, 32768), ' ', Max(-32768, -32769), ' ', Min(65535, 65536));
  WriteLn(Max(Min(5, 10), Min(3, 8)), ' ', Min(Max(5, 10), Max(3, 8)));
end.
