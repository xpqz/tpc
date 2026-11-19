program IncDecKinds;
type
  Color = (Red, Green, Blue, Black);
  Small = 1..10;
  Rec = record n: Integer; c: Char; s: String[8] end;
var
  bo: Boolean; c: Char; col: Color; sm: Small; sh: ShortInt; b: Byte;
  r: Rec; a: array[1..3] of Char; idx: Integer;
  p: ^Rec;
procedure Bump(var x: Integer; by: Integer);
begin
  Inc(x, by);
  Dec(x)
end;
procedure Outer;
var total: Integer;
  procedure Add(n: Integer);
  begin
    Inc(total, n);
    Inc(total)
  end;
begin
  total := 0;
  Add(5);
  Add(10);
  WriteLn(total)
end;
begin
  bo := False;
  Inc(bo);
  WriteLn(bo);
  Dec(bo);
  WriteLn(bo);
  c := #254;
  Inc(c);
  WriteLn(Ord(c));
  Inc(c);
  WriteLn(Ord(c));
  Dec(c);
  WriteLn(Ord(c));
  c := 'a';
  Inc(c, 2);
  WriteLn(c);
  col := Black;
  Dec(col);
  WriteLn(Ord(col));
  Dec(col, 2);
  WriteLn(Ord(col));
  Inc(col, 3);
  WriteLn(Ord(col));
  sm := 5;
  Inc(sm, 3);
  WriteLn(sm);
  Dec(sm);
  WriteLn(sm);
  sh := -128;
  Dec(sh);
  WriteLn(sh);
  Inc(sh);
  WriteLn(sh);
  b := 255;
  Inc(b);
  WriteLn(b);
  r.n := 100;
  r.c := 'x';
  Inc(r.n, 23);
  Dec(r.c);
  WriteLn(r.n, ' ', r.c);
  with r do
  begin
    Inc(n);
    Inc(c, 2)
  end;
  WriteLn(r.n, ' ', r.c);
  a[1] := 'A';
  a[2] := 'B';
  a[3] := 'C';
  for idx := 1 to 3 do Inc(a[idx], idx);
  WriteLn(a[1], a[2], a[3]);
  New(p);
  p^.n := 7;
  Inc(p^.n, 3);
  WriteLn(p^.n);
  Dispose(p);
  idx := 1;
  Bump(idx, 10);
  WriteLn(idx);
  Outer
end.
