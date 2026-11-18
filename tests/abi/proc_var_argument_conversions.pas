program ProcVarArgumentConversions;
{$F+}
{ arguments of an indirect call convert to the parameter types like a direct call }
type TP = procedure(a: Byte; b: LongInt; c: Integer);
var p: TP;
procedure Show(a: Byte; b: LongInt; c: Integer);
begin
  WriteLn(a, ' ', b, ' ', c)
end;
var i: Integer; w: Word;
begin
  p := Show;
  i := 300; w := 65535;
  p(i, i, w);
  p(w, w, 30000);
  p(255, i * 200, -1);
  p(i, w, i)
end.
