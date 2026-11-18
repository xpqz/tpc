program ProcTypePerRoutineFarDirective;
{ the far directive switch around one routine: that routine is assignable }
type TProc = procedure;
var p: TProc;
{$F+}
procedure Far1; begin WriteLn('far') end;
{$F-}
begin
  p := Far1;
  p
end.
