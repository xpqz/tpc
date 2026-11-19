program string_function_result_element_constant_index_out_of_range;
function F: string;
begin
  F := 'abc';
  F[300] := 'x';
end;
begin
  writeln(F);
end.
