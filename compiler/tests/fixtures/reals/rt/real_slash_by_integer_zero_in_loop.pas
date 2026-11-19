program RtIntZero;
var k: Integer; x: Real;
begin
  for k := 3 downto -1 do begin
    x := 6 / (k - 0);
    WriteLn(k, ' ', x:0:2)
  end
end.
