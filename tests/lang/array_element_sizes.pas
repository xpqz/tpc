program ArrayElementSizes;
var a: array[1..5] of Byte; b: array[1..5] of Integer; c: array[1..5] of LongInt;
    d: array[1..5] of Boolean; e: array[1..5] of Char; f: array[1..3] of array[1..4] of Integer;
begin
  WriteLn(SizeOf(a), ' ', SizeOf(b), ' ', SizeOf(c), ' ', SizeOf(d), ' ', SizeOf(e), ' ', SizeOf(f))
end.
