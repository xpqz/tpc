program NestedCallEnclosingChain;
{ a nested routine calls routines of every enclosing level and global routines }
var log: Integer;
procedure Mark(n: Integer);
begin
  log := log * 10 + n
end;
procedure A;
var x: Integer;
  procedure A1;
  begin
    x := x + 1;
    Mark(1)
  end;
  procedure B;
  var y: Integer;
    procedure B1;
    begin
      A1;
      y := y + 1;
      x := x + 10;
      Mark(2)
    end;
  begin
    y := 0;
    B1;
    A1;
    Mark(3);
    WriteLn('y ', y)
  end;
begin
  x := 0;
  B;
  WriteLn('x ', x)
end;
begin
  log := 0;
  A;
  WriteLn(log)
end.
