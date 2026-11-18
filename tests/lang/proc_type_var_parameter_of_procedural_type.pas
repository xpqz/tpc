program ProcTypeVarParameterOfProceduralType;
{$F+}
{ a procedural variable passed as a var parameter and assigned by the callee }
type TProc = procedure;
var p: TProc;
procedure A; begin WriteLn('A') end;
procedure B; begin WriteLn('B') end;
procedure Choose(first: Boolean; var q: TProc);
begin
  if first then q := A else q := B
end;
begin
  Choose(True, p); p;
  Choose(False, p); p
end.
