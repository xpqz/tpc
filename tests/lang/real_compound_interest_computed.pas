program RealInterest;
var balance, rate: Real; year: Integer;
begin
  balance := 1000;
  rate := 0.0625;
  for year := 1 to 10 do begin
    balance := balance * (1 + rate);
    WriteLn(year:2, balance:12:2);
  end;
  WriteLn(Round(balance * 100))
end.
