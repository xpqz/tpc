program RecursionOuterReenteredFromNested;
{ the nested routine calls its enclosing routine again: the new activation needs its own
  frame while the old nested routine still sees the old outer locals }
var trace: LongInt;
procedure Outer(level: Integer);
var mine: Integer;
  procedure Inner;
  begin
    trace := trace * 10 + mine;
    if level < 4 then Outer(level + 1);
    trace := trace * 10 + mine
  end;
begin
  mine := level;
  Inner;
  mine := mine + 5
end;
begin
  trace := 0;
  Outer(1);
  WriteLn(trace)
end.
