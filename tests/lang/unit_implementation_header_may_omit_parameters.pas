program UnitImplementationHeaderMayOmitParameters;
{ the implementation may repeat the interface header in full or write only the name }
uses Short;
begin
  WriteLn(Add(2, 3));
  WriteLn(Sub(9, 4));
  Say;
  WriteLn(Neg(5));
end.
