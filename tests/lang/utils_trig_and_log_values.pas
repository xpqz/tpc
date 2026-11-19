program UtilsTrig;
uses Utils;

procedure Row(X: Real);
begin
  WriteLn(X:0:2, ' ', Tan(X):0:5, ' ', Log10(Abs(X) + 1):0:5);
end;

procedure Inverse(X: Real);
begin
  WriteLn(X:0:2, ' ', ArcSin(X):0:5, ' ', ArcCos(X):0:5);
end;

var
  I: Integer;
begin
  Row(0);
  Row(0.5);
  Row(1);
  Row(-1);
  Row(0.7853981634);
  Row(3);
  for I := -4 to 4 do
    Inverse(I / 4);
  WriteLn(Log10(1):0:5, ' ', Log10(10):0:5, ' ', Log10(1000):0:5, ' ', Log10(0.001):0:5);
  WriteLn(Log10(2):0:5, ' ', Log10(12345):0:5);
  WriteLn(ArcSin(2 / 3):0:5, ' ', ArcCos(0.1):0:5);
end.
