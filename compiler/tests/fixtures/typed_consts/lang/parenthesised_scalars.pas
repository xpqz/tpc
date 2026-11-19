program ParenthesisedScalars;
const
  a: Integer = (1 + 2) * 3;
  b: Integer = (4);
  c: Integer = ((5));
begin
  WriteLn(a, ' ', b, ' ', c);
end.
