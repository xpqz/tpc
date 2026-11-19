program MathNames;
const
  MaxInt = 5;
var
  Sin: Integer;
  Exp: String;

function Pi: Integer;
begin
  Pi := 3;
end;

function Ln(x: Integer): Integer;
begin
  Ln := x * 2;
end;

procedure Inner;
var Cos: Boolean;
begin
  Cos := True;
  WriteLn(Cos);
end;

begin
  Sin := Pi;
  Exp := 'text';
  WriteLn(Sin, ' ', Exp, ' ', Ln(Sin), ' ', MaxInt);
  Inner;
end.
