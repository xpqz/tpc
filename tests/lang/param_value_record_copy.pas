program ParamValueRecord;
type RT = record a: Integer; b: array[1..2] of Byte; s: String[5] end;
var r: RT;
procedure Mutate(x: RT);
begin
  x.a := -1; x.b[2] := 0; x.s := 'chg';
  WriteLn(x.a, ' ', x.b[1], x.b[2], ' ', x.s)
end;
procedure MutateVar(var x: RT);
begin
  x.a := -1; x.b[2] := 0; x.s := 'chg'
end;
begin
  r.a := 5; r.b[1] := 1; r.b[2] := 2; r.s := 'orig';
  Mutate(r);
  WriteLn(r.a, ' ', r.b[1], r.b[2], ' ', r.s);
  MutateVar(r);
  WriteLn(r.a, ' ', r.b[1], r.b[2], ' ', r.s)
end.
