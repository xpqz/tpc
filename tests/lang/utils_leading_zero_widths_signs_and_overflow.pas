program UtilsLeadingZero;
uses Utils;
var
  Wide: String;
begin
  WriteLn(LeadingZero(7, 3), ' ', LeadingZero(42, 5), ' ', LeadingZero(0, 1), ' ', LeadingZero(0, 4));
  WriteLn(LeadingZero(12345, 3), ' ', LeadingZero(12345, 5), ' ', LeadingZero(12345, 6));
  WriteLn(LeadingZero(5, 0), ' ', LeadingZero(5, 1), ' ', LeadingZero(100, 2));
  WriteLn(LeadingZero(-5, 3), ' ', LeadingZero(-5, 1), ' ', LeadingZero(-5, 0), ' ', LeadingZero(-5, 6));
  WriteLn(LeadingZero(-12345, 4), ' ', LeadingZero(-12345, 7));
  WriteLn(LeadingZero(MaxLongInt, 12), ' ', LeadingZero(MaxLongInt, 10), ' ', LeadingZero(MaxLongInt, 3));
  WriteLn(LeadingZero(-MaxLongInt - 1, 13), ' ', LeadingZero(-MaxLongInt - 1, 5));
  Wide := LeadingZero(1, 255);
  WriteLn(Length(Wide), ' ', Wide[1], Wide[254], Wide[255]);
  WriteLn(LeadingZero(7, 2) + ':' + LeadingZero(5, 2) + ':' + LeadingZero(0, 2));
end.
