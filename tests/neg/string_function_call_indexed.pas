program string_function_call_indexed;
function f(i: integer): string;
begin f := 'abc' end;
var c: char;
begin
  c := f(1)[2];
end.
