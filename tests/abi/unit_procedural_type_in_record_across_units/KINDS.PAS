unit Kinds;
{$F+}
interface
uses Shape;
procedure MakeSquare(var s: TShape; side: Integer);
procedure MakeRect(var s: TShape; w, h: Integer);
implementation
function SquareArea(w, h: Integer): Integer;
begin
  SquareArea := w * w;
end;
function RectArea(w, h: Integer): Integer;
begin
  RectArea := w * h;
end;
procedure MakeSquare(var s: TShape; side: Integer);
begin
  s.name := 'square';
  s.w := side;
  s.h := side + 100;
  s.area := SquareArea;
end;
procedure MakeRect(var s: TShape; w, h: Integer);
begin
  s.name := 'rect';
  s.w := w;
  s.h := h;
  s.area := RectArea;
end;
end.
