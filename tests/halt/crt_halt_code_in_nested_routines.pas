program H7;
uses Crt;
procedure Inner(n: Integer);
begin
  WriteLn('inner ', n);
  if n = 3 then Halt(n * 7);
  Inner(n + 1);
  WriteLn('unwound ', n)
end;
begin
  Inner(1);
  WriteLn('not reached')
end.
