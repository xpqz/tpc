program H2;
var depth: Integer;
procedure Inner(n: Integer);
begin
  WriteLn('inner ', n);
  if n = 3 then Halt(0);
  Inner(n + 1);
  WriteLn('unwound ', n)
end;
procedure Outer;
begin
  WriteLn('outer');
  Inner(1);
  WriteLn('not reached')
end;
begin
  depth := 0;
  Outer;
  WriteLn('not reached')
end.
