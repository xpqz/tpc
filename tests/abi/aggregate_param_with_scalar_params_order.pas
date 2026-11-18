program AggregateParamOrder;
type RT = record a, b: Integer end;
procedure P(x: Integer; r: RT; y: Integer; var q: RT; z: LongInt);
begin
  r.a := r.a + x; q.b := q.b + y + z;
  WriteLn(x, ' ', r.a, r.b, ' ', y, ' ', q.a, q.b, ' ', z)
end;
var r, q: RT;
begin
  r.a := 1; r.b := 2; q.a := 3; q.b := 4;
  P(10, r, 20, q, 100000);
  WriteLn(r.a, r.b, ' ', q.a, ' ', q.b)
end.
