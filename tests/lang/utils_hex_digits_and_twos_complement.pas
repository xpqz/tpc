program UtilsHex;
uses Utils;
var
  Wide: String;
begin
  WriteLn('[', Hex(0, 1), '] [', Hex(0, 4), '] [', Hex(0, 0), ']');
  WriteLn('[', Hex(255, 2), '] [', Hex(255, 4), '] [', Hex(255, 1), ']');
  WriteLn('[', Hex($FFFF, 4), '] [', Hex($ABCD, 4), '] [', Hex($ABCD, 2), '] [', Hex($ABCD, 3), ']');
  WriteLn('[', Hex(-1, 4), '] [', Hex(-1, 8), '] [', Hex(-1, 2), '] [', Hex(-1, 1), ']');
  WriteLn('[', Hex(-2, 8), '] [', Hex(-256, 4), '] [', Hex(-256, 8), '] [', Hex(-32768, 4), ']');
  WriteLn('[', Hex(MaxLongInt, 8), '] [', Hex(MaxLongInt, 7), '] [', Hex(MaxLongInt, 10), ']');
  WriteLn('[', Hex(-MaxLongInt - 1, 8), '] [', Hex(-MaxLongInt - 1, 9), ']');
  WriteLn('[', Hex(65536, 4), '] [', Hex(65536, 5), '] [', Hex(1234567, 6), ']');
  WriteLn('[', Hex(16, 1), '] [', Hex(15, 1), '] [', Hex(10, 3), ']');
  Wide := Hex(1, 255);
  WriteLn(Length(Wide), ' ', Wide[254], Wide[255]);
end.
