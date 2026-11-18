program RecursionAckermann;
function Ack(m, n: Integer): Integer;
begin
  if m = 0 then Ack := n + 1
  else if n = 0 then Ack := Ack(m - 1, 1)
  else Ack := Ack(m - 1, Ack(m, n - 1))
end;
begin
  WriteLn(Ack(0, 0), ' ', Ack(1, 2), ' ', Ack(2, 3), ' ', Ack(3, 3))
end.
