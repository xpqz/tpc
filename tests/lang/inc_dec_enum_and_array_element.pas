program IncDecOrd;
type Color = (Red, Green, Blue, Black);
var c: Color; a: array[1..3] of Integer; i: Integer;
    r: record n: Integer; end; p: ^Integer;
begin
  c := Red;
  Inc(c);
  WriteLn(Ord(c));
  Inc(c);
  WriteLn(Ord(c));
  Dec(c);
  WriteLn(Ord(c));
  a[2] := 10;
  Inc(a[2]);
  Inc(a[2], 4);
  Dec(a[2], 2);
  WriteLn(a[2]);
  i := 3;
  Inc(a[i - 1], i);
  WriteLn(a[2]);
  r.n := 1;
  Inc(r.n, 41);
  WriteLn(r.n);
  New(p);
  p^ := 5;
  Inc(p^, 5);
  Dec(p^);
  WriteLn(p^);
  Dispose(p)
end.
