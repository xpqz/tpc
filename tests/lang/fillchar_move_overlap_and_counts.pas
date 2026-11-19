program FillMoveOverlap;
var
  a: array[1..6] of Byte;
  v: array[1..4] of Integer;
  n: Word;
  k: Integer;
  big: LongInt;
  i: Integer;

procedure Show;
begin
  for i := 1 to 6 do Write(a[i]);
  WriteLn;
end;

procedure Reset;
begin
  for i := 1 to 6 do a[i] := i;
end;

begin
  { memmove semantics: overlapping ranges copy as if through a buffer. }
  Reset; Move(a[1], a[2], 4); Show;
  Reset; Move(a[2], a[1], 4); Show;
  Reset; Move(a[1], a[1], 6); Show;
  Reset; Move(a[3], a[4], 3); Show;
  Reset; Move(a[4], a[3], 3); Show;
  for i := 1 to 4 do v[i] := i * 100;
  Move(v[1], v[2], 6);
  WriteLn(v[1], ' ', v[2], ' ', v[3], ' ', v[4]);
  { Counts: Word, Integer, LongInt and constant expressions are all accepted. }
  n := 3; k := 2; big := 4;
  Reset; FillChar(a, n, 0); Show;
  Reset; FillChar(a, k, 0); Show;
  Reset; FillChar(a, big, 0); Show;
  Reset; FillChar(a, SizeOf(a) div 2, 0); Show;
  Reset; FillChar(a, 0, 0); Show;
  Reset; Move(a[1], a[4], n); Show;
  Reset; Move(a[1], a[4], k + 1); Show;
  Reset; Move(a[1], a[3], big); Show;
  { A count of 0 changes nothing. }
  n := 0;
  Reset; FillChar(a, n, 9); Move(a[1], a[2], n); Show;
end.
