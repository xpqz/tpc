program FrameFunctionResultWithFrame;
{ functions that need a frame return their result after the epilogue; the result is
  correct for every scalar type and when the result variable is read in the body }
procedure Bump(var x: Integer); begin x := x + 1 end;
function I: Integer;
var t: Integer;
begin
  t := 32766;
  Bump(t); Bump(t);
  I := t - 1
end;
function B: Byte;
var t: Integer;
begin
  t := 0;
  Bump(t);
  B := 254 + t
end;
function C: Char;
var t: Integer;
begin
  t := 64;
  Bump(t);
  C := Chr(t)
end;
function L(n: LongInt): LongInt;
var t: Integer;
begin
  t := 0;
  Bump(t);
  if n <= 1 then L := t else L := n * L(n - 1)
end;
function P(n: Integer): Boolean;
var t: Integer;
begin
  t := n;
  Bump(t);
  P := Odd(t)
end;
begin
  WriteLn(I, ' ', B, ' ', C, ' ', L(10), ' ', P(1), ' ', P(2));
  WriteLn(I + I, ' ', B + B)
end.
