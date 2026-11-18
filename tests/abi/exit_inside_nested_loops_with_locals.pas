program ExitInsideNestedLoopsWithLocals;
{ Exit from a nested routine's loop while an outer local is live and modified }
procedure Outer;
var hits: Integer;
  procedure Scan(limit: Integer);
  var i, j: Integer;
  begin
    for i := 1 to limit do
      for j := 1 to limit do begin
        hits := hits + 1;
        if i * j = 12 then Exit
      end;
    hits := hits + 1000
  end;
begin
  hits := 0;
  Scan(3);
  WriteLn(hits);
  Scan(12);
  WriteLn(hits)
end;
begin
  Outer
end.
