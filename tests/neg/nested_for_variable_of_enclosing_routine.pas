program NegTest;
procedure Outer;
var v: Integer;
  procedure Inner;
  begin
    for v := 1 to 3 do Write(v)
  end;
begin
  Inner
end;
begin
end.
