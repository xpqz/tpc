program StringStateMachine;
var s: String; i, depth, maxd: Integer; ok: Boolean;
begin
  s := '((a)(b(c)))(d)';
  depth := 0; maxd := 0; ok := True;
  for i := 1 to Length(s) do
    case s[i] of
      '(': begin depth := depth + 1; if depth > maxd then maxd := depth end;
      ')': begin depth := depth - 1; if depth < 0 then ok := False end;
      'a'..'z': ;
    end;
  WriteLn(ok and (depth = 0), ' ', maxd)
end.
