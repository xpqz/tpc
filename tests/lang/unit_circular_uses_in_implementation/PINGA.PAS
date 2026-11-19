unit PingA;
interface
function Ping(n: Integer): Integer;
implementation
uses PongB;
function Ping(n: Integer): Integer;
begin
  if n = 0 then Ping := 100
  else Ping := Pong(n - 1) + 1;
end;
end.
