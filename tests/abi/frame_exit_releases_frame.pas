program FrameExitReleasesFrame;
{ Exit from a routine that has a frame, from loops and from recursion, returns to the
  caller with the stack pointer restored: later calls keep working and keep their values }
procedure Bump(var x: Integer); begin x := x + 1 end;
function Find(n: Integer): Integer;
var i, j, hits: Integer;
begin
  hits := 0;
  Find := -1;
  for i := 1 to 10 do
    for j := 1 to 10 do begin
      Bump(hits);
      if i * j = n then begin Find := i * 100 + j; Exit end
    end;
  Find := -2
end;
procedure Down(n: Integer);
var local: Integer;
begin
  local := n;
  Bump(local);
  if n = 0 then begin WriteLn('bottom'); Exit end;
  Down(n - 1);
  WriteLn('up ', n, ' ', local)
end;
procedure First;
var v: Integer;
begin
  v := 1;
  Bump(v);
  if v = 2 then Exit;
  WriteLn('not reached')
end;
var k: Integer;
begin
  WriteLn(Find(42), ' ', Find(97), ' ', Find(1));
  Down(4);
  for k := 1 to 3 do First;
  WriteLn(Find(6))
end.
