program UnitSharedDependencyInitializedOnce;
{ Left and Right both use Base; Base initializes once, before both }
uses Left, Right, Base;
begin
  WriteLn(LeftValue, ' ', RightValue);
  WriteLn(BaseCalls);
end.
