program ScopeGlobalLocalShadow;
var
  x: Integer;
  y: Integer;
procedure P;
var x: Integer;
begin
  x := 100;
  y := y + 1;
  WriteLn('P x=', x, ' y=', y)
end;
function F(y: Integer): Integer;
begin
  y := y * 2;
  F := x + y
end;
begin
  x := 1;
  y := 10;
  P;
  WriteLn('main x=', x, ' y=', y);
  WriteLn(F(5));
  WriteLn(y)
end.
