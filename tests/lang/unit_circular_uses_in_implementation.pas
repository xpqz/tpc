program UnitCircularUsesInImplementation;
{ A and B use each other in their implementation parts }
uses PingA, PongB;
begin
  WriteLn(Ping(3));
  WriteLn(Pong(3));
end.
