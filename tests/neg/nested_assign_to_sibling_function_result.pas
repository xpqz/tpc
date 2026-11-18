program NegTest;
procedure Outer;
  function F: Integer;
  begin
    F := 1
  end;
  function G: Integer;
  begin
    F := 2;
    G := 3
  end;
begin
end;
begin
end.
