program TypedconstInRecursiveProcedureIsShared;
{ one copy for all activations, not one per call }
procedure Count(n: Integer);
const
  calls: Integer = 0;
begin
  calls := calls + 1;
  if n > 0 then Count(n - 1);
  WriteLn(n, ' ', calls);
end;
begin
  Count(3);
end.
