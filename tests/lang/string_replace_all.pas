program ReplaceAll;
function Repl(s, a, b: String): String;
var p: Integer; r: String;
begin
  r := '';
  p := Pos(a, s);
  while (p > 0) and (Length(a) > 0) do begin
    r := r + Copy(s, 1, p - 1) + b;
    Delete(s, 1, p + Length(a) - 1);
    p := Pos(a, s)
  end;
  Repl := r + s
end;
begin
  WriteLn(Repl('a-b-c-d', '-', '--'));
  WriteLn(Repl('aaaa', 'aa', 'b'));
  WriteLn(Repl('hello', 'l', ''));
  WriteLn(Repl('hello', 'xyz', 'q'));
  WriteLn(Repl('', 'a', 'b'), '|')
end.
