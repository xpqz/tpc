program string_function_result_element_assigned_string;
function F: string;
begin
  F[1] := 'ab';
end;
begin
  writeln(F);
end.
