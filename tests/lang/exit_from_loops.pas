program ExitFromLoops;
{ Exit inside for, while and repeat leaves the whole routine }
procedure F(n: Integer);
var i: Integer;
begin
  for i := 1 to 10 do if i = n then begin WriteLn('for ', i); Exit end;
  i := 0;
  while True do begin i := i + 1; if i = n + 1 then begin WriteLn('while ', i); Exit end end;
end;
procedure G(n: Integer);
var i: Integer;
begin
  i := 0;
  repeat
    i := i + 1;
    if i = n then begin WriteLn('repeat ', i); Exit end
  until False
end;
begin
  F(3); F(0); G(4); F(10)
end.
