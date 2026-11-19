program UnitVariableSharedBetweenUnits;
uses Store, Writer, Reader;
begin
  WriteLn(Peek);
  Put(5);
  WriteLn(Peek);
  Put(Peek * 3);
  WriteLn(Store.Cell);
end.
