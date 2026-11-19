program RealRound;
var x: Real; l: LongInt; i: Integer;
procedure R(v: Real);
begin
  WriteLn(v:0:3, ' ', Round(v), ' ', Trunc(v), ' ', Int(v):0:1, ' ', Frac(v):0:3)
end;
begin
  R(0); R(0.25); R(0.5); R(0.75); R(1.5); R(2.5); R(-0.25); R(-0.5); R(-0.75); R(-1.5); R(-2.5);
  R(1000000.5); R(-1000000.5); R(2147483647); R(-2147483647);
  R(2147483646.5); R(32767.5); R(-32768.5); R(65535.75);
  l := Round(1.0E9 + 0.5); WriteLn(l);
  l := Trunc(-1.0E9 - 0.75); WriteLn(l);
  i := Round(100.5); WriteLn(i);
  i := Trunc(-100.99); WriteLn(i);
  x := Round(2.5); WriteLn(x:0:1);
  x := Trunc(2.9) + Round(2.5) / 2; WriteLn(x:0:2);
  WriteLn(Round(7 / 2), ' ', Trunc(7 / 2), ' ', Round(-7 / 2), ' ', Trunc(-7 / 2));
  WriteLn(Round(3), ' ', Trunc(3), ' ', Int(3):0:1, ' ', Frac(3):0:1);
  WriteLn(Round(x * 2) + 1, ' ', Trunc(x * 2) * 2);
  WriteLn(Round(Sqr(1.5)), ' ', Trunc(Abs(-2.75)))
end.
