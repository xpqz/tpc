program RangeCrash;
{$R+}
uses Crt;
type
  Small = 1..5;
var
  I: Integer;
  S: Small;
begin
  ClrScr;
  for I := 1 to 3 do
  begin
    GotoXY(I * 10, I);
    Write('step ', I);
  end;
  GotoXY(5, 20);
  I := 6;
  S := I;
  Write('not reached', S);
end.
