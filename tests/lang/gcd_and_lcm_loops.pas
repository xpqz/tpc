program GcdAndLcmLoops;
var a, b, x, y, t: Integer;
begin
  for a := 1 to 6 do
    for b := 1 to 6 do
    begin
      x := a;
      y := b;
      while y <> 0 do
      begin
        t := x mod y;
        x := y;
        y := t
      end;
      Write(x:2);
      if b = 6 then WriteLn
    end;
  WriteLn(12 * 18 div 6)
end.
