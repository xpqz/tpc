program UnitWithoutInitializationPart;
{ a unit may end with a plain end. and has no initialization code }
uses Plain;
begin
  WriteLn(Twice(21));
end.
