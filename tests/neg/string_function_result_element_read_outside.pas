program string_function_result_element_read_outside;
function G: string;
begin
  G := 'ab';
end;
var c: char;
begin
  c := G[1];
end.
