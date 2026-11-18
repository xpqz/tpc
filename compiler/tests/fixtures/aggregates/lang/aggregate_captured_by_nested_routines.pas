program AggregateCapturedByNestedRoutines;
type Vec = array[1..4] of Integer; Pt = record x, y: Integer end;
procedure Outer(n: Integer);
var v: Vec; p: Pt; i: Integer;
  procedure Mid;
  var q: Pt;
    procedure Inner;
    begin
      v[2] := v[2] + 10;
      p.x := p.x + v[1];
      q.y := p.x + p.y
    end;
  begin
    q.x := 0;
    Inner; Inner;
    v[4] := q.y;
    WriteLn(q.x, ' ', q.y)
  end;
begin
  for i := 1 to 4 do v[i] := i * n;
  p.x := 1; p.y := 2;
  Mid;
  WriteLn(v[1], ' ', v[2], ' ', v[3], ' ', v[4], ' ', p.x, ' ', p.y)
end;
begin
  Outer(1);
  Outer(5)
end.
