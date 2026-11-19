program swag_fillchar_and_move;
var
  a: array[1..4] of Byte;
  b: array[1..4] of Byte;
begin
  FillChar(a, SizeOf(a), 7);
  Move(a, b, SizeOf(a));
  writeln(b[1], b[4]);
end.
