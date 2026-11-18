program AddressOf;
type PI = ^Integer; PW = ^Word; PB = ^Byte; PR = ^Rec;
  Rec = record x: Integer; y: Integer end;
var g: Integer; arr: array[1..4] of Integer; r: Rec; p: PI; gp: Pointer;

procedure Bump(p: PI);
begin
  p^ := p^ + 1
end;

procedure Local;
var l: Integer; q: PI;
  procedure Inner;
  begin
    q^ := q^ * 2
  end;
begin
  l := 21;
  q := @l;
  Inner;
  WriteLn(l);
  Bump(@l);
  WriteLn(l)
end;

procedure ByValue(v: Integer; var w: Integer);
var a, b: PI;
begin
  a := @v; b := @w;
  a^ := a^ + 1;
  b^ := b^ + 1;
  WriteLn(v, ' ', w)
end;

begin
  g := 5;
  p := @g;
  p^ := 6;
  WriteLn(g);
  Bump(@g);
  WriteLn(g);
  arr[3] := 30;
  p := @arr[3];
  p^ := p^ + 1;
  WriteLn(arr[3]);
  r.x := 1; r.y := 2;
  p := @r.y;
  p^ := 20;
  WriteLn(r.x, ' ', r.y);
  gp := @r;
  WriteLn(PR(gp)^.y);
  gp := @g;
  WriteLn(PI(gp)^);
  Local;
  ByValue(g, g);
  WriteLn(g);
  WriteLn(@g = @g, ' ', @arr[1] <> @arr[2], ' ', @r = @r.x)
end.
