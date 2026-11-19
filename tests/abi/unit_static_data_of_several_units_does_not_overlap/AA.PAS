unit Aa;
interface
var
  LastA: Integer;
  ArrA: array[1..10] of Integer;
const
  NameA: string[8] = 'alpha';
  TagA: array[1..3] of Char = 'AAA';
procedure FillA;
function SumA: Integer;
implementation
procedure FillA;
var
  i: Integer;
begin
  for i := 1 to 10 do ArrA[i] := i;
  LastA := ArrA[10];
end;
function SumA: Integer;
var
  i, s: Integer;
begin
  s := 0;
  for i := 1 to 10 do s := s + ArrA[i];
  SumA := s;
end;
end.
