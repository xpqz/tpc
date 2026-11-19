program string_function_result_element_read_no_parameters;
function F: string;
begin
  F[0] := #2;
  F[1] := 'o';
  if F[1] = 'o' then F[2] := 'K';
end;
begin
  writeln(F);
end.
