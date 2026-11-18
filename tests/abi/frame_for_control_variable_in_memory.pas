program FrameForControlVariableInMemory;
{ a for control variable that lives in the frame (it is also passed as a var argument inside
  the loop): the loop reads and steps it in memory, up and down, to the type's limits }
procedure Peek(var x: Integer; var sum: LongInt); begin sum := sum + x end;
procedure PeekB(var x: Byte; var sum: LongInt); begin sum := sum + x end;
procedure Run;
var i: Integer; b: Byte; sum: LongInt;
begin
  sum := 0;
  for i := 1 to 5 do Peek(i, sum);
  WriteLn(sum);
  for i := 5 downto -5 do Peek(i, sum);
  WriteLn(sum);
  for i := 32766 to 32767 do Peek(i, sum);
  WriteLn(sum);
  for b := 254 to 255 do PeekB(b, sum);
  WriteLn(sum);
  for b := 1 downto 0 do PeekB(b, sum);
  WriteLn(sum, ' ', b)
end;
begin
  Run
end.
