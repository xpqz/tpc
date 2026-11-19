program IncludeInANestedRoutineDeclarationPart;
procedure Outer;
var
{$I LOCALS.INC}
begin
  a := 1;
  b := 2;
  WriteLn(a + b);
end;
begin
  Outer;
end.
