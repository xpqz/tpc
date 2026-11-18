program FrameLocalsIsolatedBetweenCalls;
{ locals of two routines called in sequence and nested; no leakage between frames }
procedure Inner(n: Integer);
var x, y: Integer;
begin
  x := n; y := n * 2;
  WriteLn('inner ', x, ' ', y)
end;
procedure Outer;
var a, b: Integer;
  procedure Nest;
  var c: Integer;
  begin
    c := 77;
    Inner(c)
  end;
begin
  a := 1; b := 2;
  Inner(a);
  Nest;
  Inner(b);
  WriteLn('outer ', a, ' ', b)
end;
begin
  Outer;
  Outer
end.
