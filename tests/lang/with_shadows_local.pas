program WithShadow;
type Rec = record x, y: Integer end;
var g: Rec; x: Integer;
procedure P;
var x: Integer; r: Rec;
begin
  x := 1; r.x := 2; r.y := 3;
  with r do begin
    WriteLn(x, ' ', y);
    x := x + 10
  end;
  WriteLn(x, ' ', r.x)
end;
begin
  x := 500; g.x := 600; g.y := 700;
  with g do WriteLn(x, ' ', y);
  WriteLn(x);
  P
end.
