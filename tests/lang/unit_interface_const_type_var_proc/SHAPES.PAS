unit Shapes;
interface
const
  MaxSides = 12;
type
  TColor = (Red, Green, Blue);
  TRect = record
    w, h: Integer;
  end;
var
  Counter: Integer;
function Area(r: TRect): Integer;
function Perimeter(r: TRect): Integer;
procedure Bump;
function Describe(c: TColor): string;
implementation
function Area(r: TRect): Integer;
begin
  Area := r.w * r.h;
end;
function Perimeter(r: TRect): Integer;
begin
  Perimeter := 2 * (r.w + r.h);
end;
procedure Bump;
begin
  Counter := Counter + 1;
end;
function Describe(c: TColor): string;
begin
  case c of
    Red: Describe := 'red';
    Green: Describe := 'green';
    Blue: Describe := 'blue';
  end;
end;
end.
