program StringManyOperandExpressions;
var s: String; i: Integer;
function Rep(c: Char; n: Integer): String;
var r: String; k: Integer;
begin
  r := '';
  for k := 1 to n do r := r + c;
  Rep := r
end;
function Wrap(open, body, close: String): String;
begin
  Wrap := open + body + close
end;
function Twice(x: String): String;
begin
  Twice := x + x
end;
begin
  WriteLn(Rep('a', 3) + Rep('b', 2) + Rep('c', 1) + '-' + Rep('d', 4) + Rep('e', 5));
  WriteLn(Length(Rep('z', 300)));
  WriteLn('1' + '2' + '3' + '4' + '5' + '6' + '7' + '8' + '9' + '0' + 'a' + 'b' + 'c' + 'd' + 'e');
  WriteLn(Wrap('[', Wrap('(', Twice(Wrap('<', 'x', '>')), ')'), ']'));
  WriteLn(Twice(Twice(Twice(Twice('ab')))));
  s := Wrap(Rep('x', 2), Rep('y', 2), Rep('z', 2)) + Twice(Rep('q', 1)) + Copy(Twice('hello'), 3, 5);
  WriteLn(s, ' ', Length(s));
  i := Length(Twice(Twice(Rep('k', 7)))) + Pos('c', Rep('a', 2) + Rep('b', 3) + 'c');
  WriteLn(i);
  WriteLn(Twice(Rep('m', 100)) + Rep('n', 100) + Rep('o', 100) = Rep('m', 200) + Rep('n', 55))
end.
