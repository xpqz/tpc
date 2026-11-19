program Indirect;
{$F+,R+}
type Small = 1..10; T = function(s: Small): Integer;
var f: T; i: Integer;
function G(s: Small): Integer;
begin
  G := s * 2
end;
begin
  f := G;
  WriteLn(f(3));
  i := 11;
  WriteLn(f(i));
  WriteLn('not reached')
end.
