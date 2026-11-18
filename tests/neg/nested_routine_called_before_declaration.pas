program NegTest;
procedure Outer;
  procedure First;
  begin
    Second
  end;
  procedure Second;
  begin
  end;
begin
  First
end;
begin
  Outer
end.
