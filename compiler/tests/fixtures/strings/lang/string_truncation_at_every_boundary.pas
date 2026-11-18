program StringTruncationAtEveryBoundary;
type S3 = String[3]; S5 = String[5];
var a: String; b: S3; c: S5;
function Cut3(s: String): S3;
begin
  Cut3 := s
end;
function Glue(x, y: S5): S5;
begin
  Glue := x + y
end;
procedure Take(s: S3; var r: String);
begin
  r := s + s
end;
procedure Grow(var s: S5);
begin
  s := s + 'ABCDEFG'
end;
begin
  a := 'abcdefgh';
  b := a; c := a;
  WriteLn(b, ' ', c, ' ', Length(b), Length(c));
  WriteLn(Cut3('uvwxyz'), Cut3(a + a), Length(Cut3('12')), Cut3(''), '|');
  WriteLn(Glue('12345', '67890'), Glue('a', 'b'), Length(Glue(a, a)));
  Take(a, a);
  WriteLn(a);
  Take('wxyz', a);
  WriteLn(a);
  c := 'ab';
  Grow(c);
  WriteLn(c, Length(c));
  b := c + c;
  c := b + c;
  WriteLn(b, c);
  c := Copy(a, 2, 100);
  b := Copy(c, 1, 2) + 'xyz';
  WriteLn(c, ' ', b)
end.
