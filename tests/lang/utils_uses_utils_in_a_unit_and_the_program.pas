program UtilsViaUnit;
uses Utils, Wrap;
begin
  WriteLn(Shout('  quiet '));
  WriteLn(Padded(7), ' ', Padded(-7));
  WriteLn(Min(3, 4), ' ', Hex(255, 4), ' ', Trim(' direct '));
end.
