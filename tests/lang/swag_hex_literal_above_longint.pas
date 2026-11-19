program swag_hex_literal_above_longint;
const
  bit: array[1..3] of LongInt = (1, 2, $80000000);
begin
  writeln(bit[1], ' ', bit[3]);
end.
