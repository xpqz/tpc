program CommentsAllStyles; { header comment }
(* paren-star comment *)
var n: Integer; { trailing }
begin
  { a comment
    spanning lines }
  n := 1; (* another
  one *)
  WriteLn(n); { comment with 'quote and (* inside }
  (* comment with { brace inside *)
  WriteLn('a { not a comment }');
  WriteLn('b (* not a comment *)')
end.
