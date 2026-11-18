program NegTest;
procedure Outer;
  procedure Inner(var a: Integer);
  begin
    a := 1
  end;
var k: Integer;
begin
  Inner(k);
  Inner(3)
end;
begin
end.
