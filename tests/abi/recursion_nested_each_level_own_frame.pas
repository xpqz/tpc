program RecursionNestedEachLevelOwnFrame;
{ recursive outer routine with a nested helper; the helper must see the locals of the
  activation that created it, not of the latest activation }
function Run(n: Integer): Integer;
var v: Integer;
  function Helper: Integer;
  begin
    Helper := v * 100 + n
  end;
begin
  v := n + 1;
  if n > 0 then
    Run := Run(n - 1) * 1000 + Helper
  else
    Run := Helper
end;
begin
  WriteLn(Run(0));
  WriteLn(Run(1));
  WriteLn(Run(2))
end.
