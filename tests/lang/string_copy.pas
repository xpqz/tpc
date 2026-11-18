program StringCopy;
var s, t: String;
begin
  s := 'Hello, World';
  WriteLn(Copy(s, 1, 5), '|', Copy(s, 8, 5), '|', Copy(s, 8, 100), '|', Copy(s, 12, 1), '|');
  WriteLn('[', Copy(s, 13, 1), '][', Copy(s, 20, 5), '][', Copy(s, 5, 0), '][', Copy(s, 1, 0), ']');
  WriteLn('[', Copy(s, 0, 3), '][', Copy(s, 12, 5), '][', Copy('', 1, 3), ']');
  t := Copy(s, 1, 255);
  WriteLn(t, Length(t));
  t := Copy(s, 3, 3);
  WriteLn(t, ' ', Length(t));
  WriteLn(Copy(Copy(s, 8, 5), 2, 3))
end.
