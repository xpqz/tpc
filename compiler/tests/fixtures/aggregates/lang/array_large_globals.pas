program ArrayLargeGlobals;
var bytes: array[0..30000] of Byte;
    longs: array[1..5000] of LongInt;
    words: array[0..999] of Word;
    i: Integer; sum: LongInt;
begin
  for i := 0 to 999 do words[i] := 65000 + i;
  for i := 1 to 5000 do longs[i] := i * 3;
  bytes[0] := 255; bytes[30000] := 200;
  sum := 0;
  for i := 1 to 5000 do sum := sum + longs[i];
  WriteLn(sum, ' ', bytes[0], ' ', bytes[30000], ' ', words[0], ' ', words[999]);
  WriteLn('text after globals')
end.
