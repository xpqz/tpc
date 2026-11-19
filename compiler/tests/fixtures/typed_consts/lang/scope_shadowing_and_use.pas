program ScopeShadowingAndUse;
type
  TPoint = record
    x, y: Integer;
  end;
const
  count: Integer = 1;
  origin: TPoint = (x: 3; y: 4);
procedure Shadow;
const
  count: Integer = 2;
begin
  count := count + 10;
  WriteLn(count);
end;
function Sum(p: TPoint): Integer;
begin
  Sum := p.x + p.y;
end;
procedure Show(var p: TPoint; n: Integer);
begin
  WriteLn(p.x, ' ', p.y, ' ', n);
end;
begin
  Shadow;
  Shadow;
  WriteLn(count, ' ', Sum(origin), ' ', Sizeof(origin), ' ', Sizeof(count));
  Show(origin, count);
  origin.x := origin.x + count;
  count := count + 1;
  Show(origin, count);
  WriteLn(Sum(origin));
end.
