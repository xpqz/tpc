program NegTest;
procedure Count(n: Integer);
begin
  if n > 0 then Count(True)
end;
begin
  Count(3)
end.
