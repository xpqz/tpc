program string_function_result_elements_assigned;
function Fill(n: integer): string;
var i: integer;
begin
  Fill := 'xyz';
  Fill[2] := 'Q';
  i := 3;
  Fill[i] := '!';
  Fill[0] := chr(n);
end;
function Upper: string;
var i: integer;
begin
  Upper[0] := #3;
  for i := 1 to 3 do Upper[i] := chr(64 + i);
end;
var s: string;
begin
  s := Fill(3);
  writeln(s, ' ', length(s));
  writeln(Upper);
end.
