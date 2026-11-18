program RealNewton;
var x, guess: Real; i: Integer;
begin
  x := 2;
  guess := 1;
  for i := 1 to 8 do begin
    guess := (guess + x / guess) / 2;
    WriteLn(i, ' ', guess:0:6);
  end;
  WriteLn(Abs(guess * guess - x) < 1E-6)
end.
