program StrReal;
var s: String; r: Real;
begin
  r := 2.5;
  Str(r, s);
  WriteLn('[', s, ']');
  Str(r:8:2, s);
  WriteLn('[', s, ']');
  Str(r:0:1, s);
  WriteLn('[', s, ']');
  Str(r:12, s);
  WriteLn('[', s, ']');
  r := -0.125;
  Str(r:0:3, s);
  WriteLn('[', s, ']');
  Str(r, s);
  WriteLn('[', s, ']');
  Str(100.0:7:1, s);
  WriteLn('[', s, ']', Length(s));
  WriteLn
end.
