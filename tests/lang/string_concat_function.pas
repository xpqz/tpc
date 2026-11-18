program StringConcatFn;
var a, b: String; t: String[5];
begin
  a := Concat('ab', 'cd', 'ef');
  WriteLn(a, ' ', Length(a));
  b := Concat(a, '-', a);
  WriteLn(b);
  a := Concat('x');
  WriteLn(a);
  a := Concat('', '');
  WriteLn(Length(a));
  t := Concat('abc', 'def');
  WriteLn(t);
  WriteLn(Concat('p', 'q'), Concat(a, 'r'))
end.
