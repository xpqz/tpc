unit Ticker;
interface
type
  TPoint = record
    x, y: Integer;
  end;
const
  Names: array[1..3] of string[4] = ('one', 'two', 'six');
  Origin: TPoint = (x: 1; y: 2);
  Greeting: string = 'hello';
var
  Total: Integer;
function Tick: Integer;
procedure Restart;
procedure Shift;
implementation
const
  Count: Integer = 0;
function Tick: Integer;
begin
  Count := Count + 1;
  Total := Total + Count;
  Tick := Count;
end;
procedure Restart;
begin
  Count := 0;
end;
procedure Shift;
begin
  Origin.x := Origin.x + 10;
  Origin.y := Origin.y + 10;
end;
begin
  Total := 0;
end.
