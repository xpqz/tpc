program AggregateVarParamElementOfAggregate;
type Rec = record a, b: Integer end; Table = array[1..3] of Rec; Mat = array[1..2, 1..2] of Integer;
var t: Table; m: Mat; one: Rec;
procedure SetInt(var x: Integer; v: Integer);
begin
  x := v
end;
procedure Swap(var a, b: Integer);
var tmp: Integer;
begin
  tmp := a; a := b; b := tmp
end;
procedure Reset(var r: Rec);
begin
  r.a := 0; r.b := 0
end;
procedure Bump(var tb: Table; i: Integer);
begin
  tb[i].a := tb[i].a + 1;
  SetInt(tb[i].b, tb[i].a * 10)
end;
begin
  t[1].a := 1; t[1].b := 2; t[2].a := 3; t[2].b := 4; t[3].a := 5; t[3].b := 6;
  SetInt(t[2].b, 40);
  Swap(t[1].a, t[3].b);
  Swap(t[1].b, t[2].a);
  WriteLn(t[1].a, t[1].b, ' ', t[2].a, t[2].b, ' ', t[3].a, t[3].b);
  Reset(t[2]); Reset(one);
  Bump(t, 2); Bump(t, 2); Bump(t, 3);
  WriteLn(t[2].a, ' ', t[2].b, ' ', t[3].a, ' ', t[3].b, ' ', one.a);
  m[1, 1] := 1; m[1, 2] := 2; m[2, 1] := 3; m[2, 2] := 4;
  Swap(m[1, 2], m[2, 1]);
  SetInt(m[2][2], m[1][1] + 100);
  WriteLn(m[1, 1], m[1, 2], m[2, 1], ' ', m[2, 2])
end.
