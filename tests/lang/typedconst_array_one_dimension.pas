program TypedconstArrayOneDimension;
const
  primes: array[1..6] of Integer = (2, 3, 5, 7, 11, 13);
  letters: array[0..3] of Char = ('a', 'b', 'c', 'd');
  flags: array[False..True] of string[5] = ('no', 'yes');
var
  i, sum: Integer;
begin
  sum := 0;
  for i := 1 to 6 do
    sum := sum + primes[i];
  WriteLn(sum);
  for i := 0 to 3 do
    Write(letters[i]);
  WriteLn;
  WriteLn(flags[False], ' ', flags[True]);
  primes[1] := 100;
  letters[0] := 'z';
  WriteLn(primes[1], ' ', letters[0]);
end.
