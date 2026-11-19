program string_function_result_element_read_with_parameters;
function F(n: integer): string;
begin
  F := 'xyz';
  F[2] := 'Q';
  writeln(F[1]);
end;
begin
  writeln(F(1));
end.
