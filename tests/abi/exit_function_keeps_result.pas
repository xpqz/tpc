program ExitFunctionKeepsResult;
{ the result assigned before Exit is what the caller receives }
function Classify(n: Integer): Integer;
begin
  Classify := -1;
  if n < 0 then Exit;
  Classify := 0;
  if n = 0 then Exit;
  Classify := 1;
  if n < 10 then Exit;
  Classify := 2
end;
function Late(n: Integer): LongInt;
var i: Integer; s: LongInt;
begin
  s := 0;
  Late := 0;
  for i := 1 to n do begin
    s := s + i;
    Late := s;
    if s > 20 then Exit
  end;
  Late := s * 1000
end;
var i: Integer;
begin
  for i := -2 to 12 do Write(Classify(i), ' ');
  WriteLn;
  WriteLn(Late(3), ' ', Late(6), ' ', Late(100))
end.
