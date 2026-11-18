program StrShort;
var s: String[3]; t: String;
begin
  Str(12345, s);
  WriteLn('[', s, ']', Length(s));
  Str(7, s);
  WriteLn('[', s, ']', Length(s));
  Str(-12, s);
  WriteLn('[', s, ']');
  Str(5:8, s);
  WriteLn('[', s, ']');
  Str(123456789, t);
  t := t + '!';
  WriteLn(t, ' ', Length(t))
end.
