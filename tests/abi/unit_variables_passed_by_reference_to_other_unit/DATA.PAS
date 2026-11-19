unit Data;
interface
type
  TPoint = record
    x, y: Integer;
  end;
  TCells = array[1..3] of Char;
var
  Counter: Integer;
  Title: string;
  Point: TPoint;
  Cells: TCells;
procedure Init;
implementation
procedure Init;
begin
  Counter := 3;
  Title := 'draft';
  Point.x := 1;
  Point.y := 2;
  Cells[1] := '.';
  Cells[2] := '.';
  Cells[3] := '.';
end;
end.
