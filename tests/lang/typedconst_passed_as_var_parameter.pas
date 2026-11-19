program TypedconstPassedAsVarParameter;
type
  TPoint = record
    x, y: Integer;
  end;
const
  n: Integer = 5;
  p: TPoint = (x: 1; y: 2);
  s: string = 'abc';
procedure Bump(var v: Integer);
begin
  v := v + 10;
end;
procedure Swap(var a: TPoint);
var
  t: Integer;
begin
  t := a.x;
  a.x := a.y;
  a.y := t;
end;
procedure Grow(var t: string);
begin
  t := t + '!';
end;
begin
  Bump(n);
  Bump(p.x);
  Swap(p);
  Grow(s);
  WriteLn(n, ' ', p.x, ' ', p.y, ' ', s);
end.
