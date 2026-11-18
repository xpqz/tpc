program ExitNestedLoopsInProcedure;
{ Exit from inside for, while and repeat loops nested in a procedure: the frame is
  released, later statements in the procedure are skipped }
procedure Find(target: Integer);
var i, j, k: Integer;
begin
  for i := 1 to 10 do
    for j := 1 to 10 do begin
      k := 0;
      while k < 10 do begin
        repeat
          k := k + 1;
          if i * 100 + j * 10 + k = target then begin
            WriteLn('found ', i, ' ', j, ' ', k);
            Exit
          end
        until k mod 3 = 0
      end
    end;
  WriteLn('not found ', target)
end;
begin
  Find(111);
  Find(555);
  Find(1000);
  Find(1099)
end.
