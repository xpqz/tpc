program RealCalls;
function Sq(x: Real): Real; begin Sq := x * x; end;
function Add(a, b: Real): Real; begin Add := a + b; end;
function Id(x: Real): Real; begin Id := x; end;
procedure Show(x: Real; i: Integer; y: Real);
begin
  WriteLn(x:0:2, ' ', i, ' ', y:0:2);
end;
var i: Integer;
begin
  WriteLn(Add(Sq(3), Sq(Add(1, 0.5))):0:2);
  Show(Id(1.5), 2, Id(Sq(0.5)));
  for i := 1 to 3 do Show(Sq(i), i, Add(i, Id(0.25)));
  WriteLn(Id(Id(Id(2))) + Id(1):0:1)
end.
