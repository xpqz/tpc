program IncludeInsideAUnit;
uses Wrapped;
begin
  WriteLn(Wrapped.Limit);
  WriteLn(Clamp(500));
  WriteLn(Clamp(-5));
  WriteLn(Clamp(50));
end.
