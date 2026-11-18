program ProcVarModifiesGlobals;
{$F+}
{ callbacks run through a procedural parameter and change global state }
type TStep = procedure(i: Integer);
var sum, prod: LongInt;
procedure AddStep(i: Integer); begin sum := sum + i end;
procedure MulStep(i: Integer); begin prod := prod * i end;
procedure ForEach(lo, hi: Integer; step: TStep);
var i: Integer;
begin
  for i := lo to hi do step(i)
end;
begin
  sum := 0; prod := 1;
  ForEach(1, 10, AddStep);
  ForEach(1, 10, MulStep);
  WriteLn(sum, ' ', prod);
  ForEach(5, 4, AddStep);
  WriteLn(sum)
end.
