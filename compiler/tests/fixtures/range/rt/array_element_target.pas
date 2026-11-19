program ElemTarget;
{$R+}
var a: array[1..3] of Byte; i: Integer;
begin
  i := 255;
  a[2] := i;
  WriteLn(a[2]);
  i := 256;
  a[3] := i;
  WriteLn('not reached')
end.
