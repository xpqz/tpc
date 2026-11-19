program LocalTarget;
{$R+}
procedure P(n: Integer);
var b: Byte;
begin
  b := n;
  WriteLn(b);
end;
begin
  P(255);
  P(256);
  WriteLn('not reached')
end.
