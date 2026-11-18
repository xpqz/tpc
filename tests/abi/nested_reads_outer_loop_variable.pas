program NestedReadsOuterLoopVariable;
{ a for control variable read by a nested routine lives in the frame and is stepped there;
  the nested routine sees the current value on every iteration }
procedure Outer;
var i: Integer; sum: LongInt;
  procedure Show;
  begin
    sum := sum + i;
    Write(i, ' ')
  end;
begin
  sum := 0;
  for i := 1 to 4 do Show;
  WriteLn;
  for i := 4 downto 1 do Show;
  WriteLn(sum)
end;
begin
  Outer
end.
