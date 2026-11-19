program FuncRes;
{$R+}
var b: Byte; i: Integer;
function F(x: Integer): Byte;
begin
  F := x;
end;
begin
  WriteLn('before');
  i := 255;
  WriteLn(F(i));
  i := 256;
  b := F(i);
  WriteLn('not reached')
end.
