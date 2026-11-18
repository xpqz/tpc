program ArrayTypes;
var a: array[1..4] of Byte; b: array[1..4] of ShortInt; c: array[1..4] of Word;
    d: array[1..4] of LongInt; e: array[1..4] of Boolean; f: array[1..4] of Char; i: Integer;
begin
  for i := 1 to 4 do begin
    a[i] := 250 + i * 2;
    b[i] := -120 - i * 3;
    c[i] := 65530 + i;
    d[i] := 2000000000 + i * 100000000;
    e[i] := Odd(i);
    f[i] := Chr(64 + i)
  end;
  for i := 1 to 4 do WriteLn(a[i], ' ', b[i], ' ', c[i], ' ', d[i], ' ', e[i], ' ', f[i])
end.
