program RangeString;
{$R+}
var s: String[3]; t: String; c: Char; i: Integer;
begin
  s := 'abc';
  for i := 0 to 3 do begin
    c := s[i];
    Write(Ord(c), ' ');
  end;
  WriteLn;
  i := 3;
  s[i] := 'z';
  s[1] := 'y';
  WriteLn(s);
  t := 'hello';
  i := 255;
  c := t[i];
  i := 0;
  t[i] := Chr(2);
  WriteLn(t)
end.
