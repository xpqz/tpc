program string_function_result_element_assigned_integer;
function F: string;
begin
  F[1] := 5;
end;
begin
  writeln(F);
end.
