program SetSieveByte;
var composite: set of Byte; i, j, n, last: Integer;
begin
  composite := [0, 1];
  for i := 2 to 15 do
    if not (i in composite) then begin
      j := i * i;
      while j <= 255 do begin
        composite := composite + [j];
        j := j + i;
      end;
    end;
  n := 0;
  last := 0;
  for i := 0 to 255 do
    if not (i in composite) then begin
      n := n + 1;
      last := i;
    end;
  WriteLn(n, ' ', last)
end.
