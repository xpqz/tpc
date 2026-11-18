program NegTest;
type R = record x: Integer; end;
var p: ^R;
begin
  p^.y := 1;
end.
