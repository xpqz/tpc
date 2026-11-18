program RecursionGcdPowerDigits;
function Gcd(a, b: Integer): Integer;
begin
  if b = 0 then Gcd := a else Gcd := Gcd(b, a mod b)
end;
function Power(b, e: Integer): LongInt;
begin
  if e = 0 then Power := 1
  else Power := b * Power(b, e - 1)
end;
function DigitSum(n: LongInt): Integer;
begin
  if n = 0 then DigitSum := 0
  else DigitSum := n mod 10 + DigitSum(n div 10)
end;
procedure Binary(n: Integer);
begin
  if n > 1 then Binary(n div 2);
  Write(n mod 2)
end;
begin
  WriteLn(Gcd(48, 18), ' ', Gcd(17, 5), ' ', Gcd(100, 0));
  WriteLn(Power(2, 15), ' ', Power(3, 10), ' ', Power(7, 0));
  WriteLn(DigitSum(0), ' ', DigitSum(12345), ' ', DigitSum(2147483647));
  Binary(37);
  WriteLn;
  Binary(1);
  WriteLn;
  Binary(1000);
  WriteLn
end.
