program SetSieve;
var sieve, primes: set of 2..255; i, j, n: Integer;
begin
  sieve := [2..255];
  primes := [];
  for i := 2 to 255 do
    if i in sieve then begin
      primes := primes + [i];
      j := i * i;
      while j <= 255 do begin
        sieve := sieve - [j];
        j := j + i;
      end;
    end;
  n := 0;
  for i := 2 to 255 do
    if i in primes then begin
      n := n + 1;
      Write(i, ' ');
      if n mod 10 = 0 then WriteLn;
    end;
  WriteLn;
  WriteLn(n)
end.
