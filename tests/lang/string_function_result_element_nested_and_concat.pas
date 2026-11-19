program string_function_result_element_nested_and_concat;
function G(c: char): string;
  procedure SetFirst;
  begin
    G[1] := c;
  end;
begin
  G := 'abc';
  SetFirst;
  G[3] := UpCase(c);
end;
function Chain: string;
begin
  Chain[0] := #2;
  Chain[1] := 'h';
  Chain[2] := 'i';
end;
var s: string;
begin
  s := G('x');
  writeln(s);
  s := Chain + '!';
  writeln(s, length(s));
end.
