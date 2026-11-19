program IncludeInARoutineHeading;
procedure Show
{$I PARAMS.INC}
begin
  WriteLn(a, ' ', b);
end;
begin
  Show(1, 2);
end.
