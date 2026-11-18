program FunctionCallsNestedInConditions;
{ function results in nested calls and conditions }
function Sq(x: Integer): Integer; begin Sq := x * x end;
function IsPos(x: Integer): Boolean; begin IsPos := x > 0 end;
begin
  if IsPos(Sq(-3)) then WriteLn(Sq(Sq(2)));
  if not IsPos(Sq(0)) then WriteLn('zero')
end.
