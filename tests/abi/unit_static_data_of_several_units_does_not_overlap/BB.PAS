unit Bb;
interface
var
  LastB: Integer;
  ArrB: array[1..20] of Integer;
const
  NameB: string[8] = 'beta';
  TagB: array[1..3] of Char = 'BBB';
procedure FillB;
function SumB: Integer;
implementation
procedure FillB;
var
  i: Integer;
begin
  for i := 1 to 20 do ArrB[i] := i * 2;
  LastB := ArrB[20];
end;
function SumB: Integer;
var
  i, s: Integer;
begin
  s := 0;
  for i := 1 to 20 do s := s + ArrB[i];
  SumB := s;
end;
end.
