program ProcCallsProc;
procedure Star;
begin
  Write('*')
end;
procedure Line(n: Integer);
var i: Integer;
begin
  for i := 1 to n do Star;
  WriteLn
end;
procedure Triangle(n: Integer);
var i: Integer;
begin
  for i := 1 to n do Line(i)
end;
begin
  Triangle(4);
  Line(0);
  Line(7)
end.
