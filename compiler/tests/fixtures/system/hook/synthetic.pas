program Synthetic;
var local: Integer;
begin
  SysV := SysK * 2;
  local := SysSquare(3);
  SysShow(SysV);
  SysShow(local);
  WriteLn(SysK + SysV, ' ', Abs(-local))
end.
