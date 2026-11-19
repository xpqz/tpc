program string_function_result_element_assigned_outside;
function F: string;
begin
  F := 'abc';
end;
begin
  F[1] := 'x';
end.
