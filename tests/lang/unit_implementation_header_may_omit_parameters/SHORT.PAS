unit Short;
interface
function Add(a, b: Integer): Integer;
function Sub(a, b: Integer): Integer;
procedure Say;
function Neg(n: Integer): Integer;
implementation
function Add(a, b: Integer): Integer;
begin
  Add := a + b;
end;
function Sub;
begin
  Sub := a - b;
end;
procedure Say;
begin
  WriteLn('said');
end;
function Neg;
begin
  Neg := -n;
end;
end.
