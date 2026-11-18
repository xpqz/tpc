program BooleanShortCircuit;
var i: Integer;
function Noisy(b: Boolean): Boolean;
begin
  Write('N');
  Noisy := b
end;
begin
  i := 0;
  if (i <> 0) and (10 div i > 1) then
    WriteLn('yes')
  else
    WriteLn('no');
  if (i = 0) or (10 div i > 1) then
    WriteLn('yes')
  else
    WriteLn('no');
  WriteLn(Noisy(False) and Noisy(True));
  WriteLn(Noisy(True) or Noisy(False));
  WriteLn(Noisy(True) and Noisy(True))
end.
