unit Vec;
interface
type
  TVec = record
    x, y: Integer;
  end;
  TVector = array[1..4] of Integer;
procedure Sum(a, b: TVec; var r: TVec);
procedure Scale(var v: TVec; k: Integer);
function Dot(a, b: TVec): Integer;
function Total(v: TVector): Integer;
procedure Reverse(var v: TVector);
implementation
procedure Sum(a, b: TVec; var r: TVec);
begin
  r.x := a.x + b.x;
  r.y := a.y + b.y;
end;
procedure Scale(var v: TVec; k: Integer);
begin
  v.x := v.x * k;
  v.y := v.y * k;
end;
function Dot(a, b: TVec): Integer;
begin
  a.x := a.x * b.x;
  Dot := a.x + a.y * b.y;
end;
function Total(v: TVector): Integer;
var
  i, t: Integer;
begin
  t := 0;
  for i := 1 to 4 do
    t := t + v[i];
  v[1] := 0;
  Total := t;
end;
procedure Reverse(var v: TVector);
var
  i, t: Integer;
begin
  for i := 1 to 2 do
  begin
    t := v[i];
    v[i] := v[5 - i];
    v[5 - i] := t;
  end;
end;
end.
