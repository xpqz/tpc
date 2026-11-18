program PrimesTrialDivision;
var n, d, count: Integer;
    prime: Boolean;
begin
  count := 0;
  for n := 2 to 200 do
  begin
    prime := True;
    d := 2;
    while (d * d <= n) and prime do
    begin
      if n mod d = 0 then prime := False;
      d := d + 1
    end;
    if prime then
    begin
      Write(n:4);
      count := count + 1;
      if count mod 10 = 0 then WriteLn
    end
  end;
  WriteLn;
  WriteLn(count, ' primes')
end.
