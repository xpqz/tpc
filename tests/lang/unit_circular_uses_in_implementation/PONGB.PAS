unit PongB;
interface
function Pong(n: Integer): Integer;
implementation
uses PingA;
function Pong(n: Integer): Integer;
begin
  if n = 0 then Pong := 200
  else Pong := Ping(n - 1) + 1;
end;
end.
