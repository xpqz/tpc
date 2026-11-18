program VarParamReadBeforeWrite;
{ the callee sees the caller's current value }
procedure Double(var x: LongInt);
begin
  x := x * 2
end;
var v: LongInt; i: Integer;
begin
  v := 3;
  for i := 1 to 5 do Double(v);
  WriteLn(v)
end.
