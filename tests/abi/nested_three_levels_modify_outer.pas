program NestedThreeLevelsModifyOuter;
{ three-level nesting; the innermost routine modifies a local of the outermost }
var shown: Integer;
procedure A;
var x: Integer;
  procedure B;
  var y: Integer;
    procedure C;
    begin
      x := x + 100;
      y := y + 10
    end;
  begin
    y := 1;
    C;
    C;
    shown := y
  end;
begin
  x := 5;
  B;
  WriteLn(x, ' ', shown)
end;
begin
  A
end.
