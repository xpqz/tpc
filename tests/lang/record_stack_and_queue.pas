program RecordStack;
type Stack = record top: Integer; data: array[1..8] of Integer end;
var s: Stack; i: Integer;
procedure Push(var s: Stack; v: Integer);
begin
  with s do begin top := top + 1; data[top] := v end
end;
function Pop(var s: Stack): Integer;
begin
  with s do begin Pop := data[top]; top := top - 1 end
end;
begin
  s.top := 0;
  for i := 1 to 8 do Push(s, i * i);
  for i := 1 to 5 do Write(Pop(s), ' ');
  WriteLn(s.top)
end.
