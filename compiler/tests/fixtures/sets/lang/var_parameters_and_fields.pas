program SetVarParams;
type BS = set of Byte;
  Rec = record n: Integer; s: BS; m: Integer end;
var g: BS; r: Rec; arr: array[0..2] of BS; i, k: Integer;
procedure AddRange(var s: BS; lo, hi: Integer);
var j: Integer;
begin
  for j := lo to hi do s := s + [j]
end;
procedure Clear(var s: BS);
begin
  s := []
end;
procedure Swap2(var x, y: BS);
var t: BS;
begin
  t := x; x := y; y := t
end;
function Total(var s: BS): Integer;
var j, n: Integer;
begin
  n := 0;
  for j := 0 to 255 do if j in s then n := n + j;
  Total := n
end;
procedure Outer;
var loc: BS;
  procedure Inner;
  begin
    loc := loc + [200, 201];
    g := g + loc
  end;
begin
  loc := [1];
  Inner;
  WriteLn(Total(loc), ' ', Total(g))
end;
begin
  Clear(g); AddRange(g, 3, 5);
  WriteLn(Total(g));
  r.n := 7; r.m := 9; Clear(r.s); AddRange(r.s, 10, 12);
  WriteLn(r.n, ' ', Total(r.s), ' ', r.m);
  for i := 0 to 2 do begin Clear(arr[i]); AddRange(arr[i], i, i + 2) end;
  Swap2(arr[0], arr[2]);
  for i := 0 to 2 do WriteLn(Total(arr[i]));
  Swap2(g, r.s);
  WriteLn(Total(g), ' ', Total(r.s));
  Outer;
  k := 0;
  if 12 in r.s + g then k := 1;
  WriteLn(k)
end.
