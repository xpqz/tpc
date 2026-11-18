program NestedExitWithFrames;
{ Exit from nested routines that have frames (static link slot plus captured locals), from
  loops and recursion: every frame is released, results and outer values survive }
procedure Bump(var x: Integer); begin x := x + 1 end;
function Search(limit: Integer): Integer;
var found: Integer;
  procedure Scan(from: Integer);
  var probe: Integer;
  begin
    probe := from;
    while probe <= limit do begin
      Bump(probe);
      if probe * probe > 50 then begin found := probe; Exit end
    end;
    found := -1
  end;
begin
  found := 0;
  Scan(1);
  Search := found
end;
procedure Down(n: Integer);
var depth: Integer;
  procedure Mark;
  begin
    Bump(depth);
    if depth > 3 then Exit;
    WriteLn('mark ', n, ' ', depth)
  end;
begin
  depth := n;
  Mark;
  if n = 0 then Exit;
  Down(n - 1);
  WriteLn('back ', n, ' ', depth)
end;
begin
  WriteLn(Search(100), ' ', Search(5));
  Down(5)
end.
