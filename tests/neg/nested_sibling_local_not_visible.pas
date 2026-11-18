program NegTest;
procedure Outer;
  procedure A;
  var a1: Integer;
  begin
    a1 := 1
  end;
  procedure B;
  begin
    a1 := 2
  end;
begin
  A;
  B
end;
begin
  Outer
end.
