program WhileNested;
var i, j: Integer;
begin
  i := 1;
  while i <= 4 do
  begin
    j := 1;
    while j <= i do
    begin
      Write(i * j:3);
      j := j + 1
    end;
    WriteLn;
    i := i + 1
  end
end.
