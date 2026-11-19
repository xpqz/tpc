program Str255;
{$R+}
var t: String; i: Integer;
begin
  t := 'x';
  i := 255;
  WriteLn(Ord(t[i]) * 0);
  i := 256;
  WriteLn(t[i]);
  WriteLn('not reached')
end.
