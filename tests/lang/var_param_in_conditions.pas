program VarParamInConditions;
{ a var parameter as the operand of while, repeat, if, case, for bounds and the
  short-circuit operators }
procedure Count(var n: Integer; var flag: Boolean);
var total: Integer;
begin
  total := 0;
  while n > 0 do begin total := total + n; n := n - 1 end;
  repeat n := n + 3 until n >= 7;
  if (n > 5) and flag then WriteLn('both') else WriteLn('not both');
  if (n < 5) or flag then WriteLn('either');
  case n of
    7: WriteLn('seven');
    8..20: WriteLn('eight or more')
  else WriteLn('other')
  end;
  for total := n to n + 2 do Write(total, ' ');
  WriteLn;
  flag := not flag
end;
var k: Integer; t: Boolean;
begin
  k := 4; t := True;
  Count(k, t);
  WriteLn(k, ' ', t);
  k := 0; t := False;
  Count(k, t);
  WriteLn(k, ' ', t)
end.
