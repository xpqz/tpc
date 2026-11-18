program ExitFromRecursion;
{ Exit at the bottom of a recursion unwinds one level at a time; frames are popped }
procedure Down(n: Integer);
begin
  if n = 0 then begin
    WriteLn('bottom');
    Exit
  end;
  Down(n - 1);
  Write(n, ' ')
end;
function Find(n, goal: Integer): Integer;
begin
  Find := -1;
  if n > 800 then Exit;
  if n = goal then begin Find := n; Exit end;
  Find := Find(n + 1, goal)
end;
begin
  Down(8);
  WriteLn;
  WriteLn(Find(0, 700), ' ', Find(0, 900), ' ', Find(5, 5))
end.
