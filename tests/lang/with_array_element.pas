program WithElement;
type R = record v, w: Integer end;
var t: array[1..3] of R; k, i: Integer;
function Bump: Integer; begin k := k + 1; Bump := k end;
begin
  k := 1;
  for i := 1 to 3 do begin t[i].v := 0; t[i].w := 0 end;
  with t[Bump] do begin v := 5; w := 6; k := 3; v := v + 1 end;
  WriteLn(t[1].v, ' ', t[2].v, ' ', t[2].w, ' ', t[3].v, ' ', k);
  with t[1], t[3] do begin v := 7; w := 8 end;
  WriteLn(t[1].v, t[1].w, t[3].v, t[3].w)
end.
