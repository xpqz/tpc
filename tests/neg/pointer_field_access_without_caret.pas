program NegTest;
type R = record x: Integer; end;
var p: ^R;
begin
  p.x := 1;
end.
