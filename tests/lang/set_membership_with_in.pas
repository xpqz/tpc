program SetIn;
var s: set of Byte; i: Integer; c: Char; b: Byte;
begin
  s := [1, 10, 100, 200, 255, 0];
  for i := 0 to 255 do
    if i in s then Write(i, ' ');
  WriteLn;
  b := 100;
  WriteLn(b in s, ' ', 101 in s, ' ', 255 in s, ' ', 0 in s);
  WriteLn(not (7 in s), ' ', not (10 in s));
  c := 'k';
  WriteLn(c in ['a'..'z'], ' ', c in ['A'..'Z'], ' ', c in ['k']);
  WriteLn(3 in [1..5], ' ', 6 in [1..5], ' ', 0 in [1..5])
end.
