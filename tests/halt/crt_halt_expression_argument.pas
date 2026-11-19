program H12;
uses Crt;
const base = 20;
var i: Integer;
begin
  for i := 1 to 5 do
    if i * i > 10 then Halt(base + i);
  WriteLn('not reached')
end.
