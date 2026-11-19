function Clamp(n: Integer): Integer;
begin
  if n > Limit then Clamp := Limit
  else if n < Floor then Clamp := Floor
  else Clamp := n;
end;
