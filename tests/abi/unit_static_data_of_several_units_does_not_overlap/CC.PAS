unit Cc;
interface
var
  LastC: Integer;
  ArrC: array[1..5] of Integer;
const
  NameC: string[8] = 'gamma';
  TagC: array[1..3] of Char = 'CCC';
procedure FillC;
function SumC: Integer;
implementation
procedure FillC;
var
  i: Integer;
begin
  for i := 1 to 5 do ArrC[i] := i * 100;
  LastC := ArrC[5];
end;
function SumC: Integer;
var
  i, s: Integer;
begin
  s := 0;
  for i := 1 to 5 do s := s + ArrC[i];
  SumC := s;
end;
end.
