program VarParamLocalAndValueParamArguments;
{ locals and value parameters of the caller passed as var arguments live in memory and
  must keep their updates }
procedure Inc2(var x: Integer);
begin
  x := x + 2
end;
procedure Caller(v: Integer);
var loc: Integer;
begin
  loc := v * 10;
  Inc2(loc);
  Inc2(v);
  Inc2(loc);
  WriteLn(v, ' ', loc)
end;
var arg: Integer;
begin
  arg := 5;
  Caller(arg);
  WriteLn(arg)
end.
