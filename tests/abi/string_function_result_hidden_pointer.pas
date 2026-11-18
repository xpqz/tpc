program StringResultPtr;
function Rep(c: Char; n: Integer): String;
var s: String; i: Integer;
begin
  s := '';
  for i := 1 to n do s := s + c;
  Rep := s
end;
function Twice(s: String): String; begin Twice := s + s end;
var a, b: String; i: Integer;
begin
  for i := 1 to 5 do begin
    a := Rep('a', i);
    b := Twice(Rep('b', i)) + Twice(a);
    WriteLn(b, ' ', Length(b))
  end;
  WriteLn(Twice(Twice(Twice('x'))))
end.
