program swag_string_function_result_index;
function Pad: string;
begin
  Pad[0] := #3;
  Pad[1] := 'a';
  Pad[2] := 'b';
  Pad[3] := 'c';
end;
begin
  writeln(Pad);
end.
