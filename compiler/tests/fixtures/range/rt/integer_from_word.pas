program IntWord;
{$R+}
var i: Integer; w: Word;
begin
  w := 32767;
  i := w;
  WriteLn(i);
  w := 32768;
  i := w;
  WriteLn('not reached')
end.
