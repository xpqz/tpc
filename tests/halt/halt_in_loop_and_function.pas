program H3;
function Check(i: Integer): Integer;
begin
  if i = 4 then Halt(0);
  Check := i * 10
end;
var i: Integer;
begin
  for i := 1 to 10 do
    WriteLn(Check(i))
end.
