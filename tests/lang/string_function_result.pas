program StringFunction;
function Rep(c: Char; n: Integer): String;
var s: String; i: Integer;
begin
  s := '';
  for i := 1 to n do s := s + c;
  Rep := s
end;
function Rev(s: String): String;
var r: String; i: Integer;
begin
  r := '';
  for i := Length(s) downto 1 do r := r + s[i];
  Rev := r
end;
function Pad(s: String; w: Integer): String;
begin
  while Length(s) < w do s := ' ' + s;
  Pad := s
end;
begin
  WriteLn(Rep('*', 5), '|', Rep('-', 0), '|', Rev('stressed'), '|', Pad('7', 4));
  WriteLn(Rev(Rev('abc')), ' ', Length(Rep('z', 100)), ' ', Rev(Pad('ab', 5)), '|');
  WriteLn(Rep('a', 3) + Rep('b', 2) = 'aaabb')
end.
