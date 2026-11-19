program UtilsShadowed;
uses Utils;

function Power(Base, Exponent: Real): Real;
begin
  Power := -1;
end;

function Max(A, B: LongInt): LongInt;
begin
  Max := 1000 * A + B;
end;

var
  Trim: Integer;
begin
  WriteLn(Power(2, 3):0:1);
  WriteLn(Max(4, 5));
  WriteLn(Min(4, 5));
  Trim := 3;
  WriteLn(Trim);
  WriteLn(Utils.Trim('  unit  '), '|', Utils.Max(4, 5), '|', Utils.Power(2, 3):0:1);
  WriteLn(UpperCase('mixed'));
end.
