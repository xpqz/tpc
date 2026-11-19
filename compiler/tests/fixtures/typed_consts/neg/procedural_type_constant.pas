program ProceduralTypeConstant;
type
  TProc = procedure;
procedure Hello;
begin
end;
const
  p: TProc = Hello;
begin
end.
