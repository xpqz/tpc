program NestedRoutineWithoutCapturedVariables;
{ routines with nested routines that capture nothing, and nested routines that use only
  globals and their own variables, at several depths; links are still passed correctly }
var g: Integer;
procedure A;
  procedure B;
    procedure C;
    begin
      g := g + 100
    end;
  var own: Integer;
  begin
    own := 5;
    g := g + own;
    C
  end;
begin
  g := g + 1;
  B
end;
function F(n: Integer): Integer;
  function Twice(k: Integer): Integer;
  begin
    Twice := k * 2
  end;
begin
  F := Twice(n) + Twice(1)
end;
begin
  g := 0;
  A;
  WriteLn(g);
  A;
  WriteLn(g, ' ', F(20))
end.
