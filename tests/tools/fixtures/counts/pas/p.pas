program P;
{ brace comment }
(* paren comment *)
{ multi
  line brace }

(* multi
   line paren *)
{$I+}
{$R-,S-}
   
	
var x: integer; { trailing brace }
begin
  x := 1; (* trailing paren *)
  { lead } x := 2;
  (* lead *) x := 3;
{ start
  end } x := 4;
  x := 5; { start

  end }
{ a } { b }
(* a *) { b } (* c *)
end.
