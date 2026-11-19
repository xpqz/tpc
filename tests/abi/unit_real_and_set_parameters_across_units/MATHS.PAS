unit Maths;
interface
type
  TSmallSet = set of 0..31;
function Hypot2(a, b: Real): Real;
function Half(n: Integer): Real;
function Count(s: TSmallSet): Integer;
procedure Odds(var s: TSmallSet; limit: Integer);
procedure Grow(var s: TSmallSet);
implementation
function Hypot2(a, b: Real): Real;
begin
  Hypot2 := a * a + b * b;
end;
function Half(n: Integer): Real;
begin
  Half := n / 2;
end;
function Count(s: TSmallSet): Integer;
var
  i, n: Integer;
begin
  n := 0;
  for i := 0 to 31 do
    if i in s then n := n + 1;
  Count := n;
end;
procedure Odds(var s: TSmallSet; limit: Integer);
var
  i: Integer;
begin
  for i := 0 to limit - 1 do
    s := s - [i];
end;
procedure Grow(var s: TSmallSet);
begin
  s := s + [10, 11, 12];
end;
end.
