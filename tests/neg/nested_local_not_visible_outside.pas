program NegTest;
procedure Outer;
var hidden: Integer;
  procedure Inner;
  begin
    hidden := 1
  end;
begin
  Inner
end;
begin
  hidden := 2
end.
