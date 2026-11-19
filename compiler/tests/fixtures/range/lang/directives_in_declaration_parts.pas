program DeclDirectives;
{$R+}
const K = 5;
{$R-}
type Small = 1..10;
{$R+}
var s: Small;
{$R-}
procedure P(n: Integer);
{$R+}
var b: Byte;
{$R-}
begin
  b := n;
  WriteLn(b);
end;
begin
  P(300);
  s := K;
  WriteLn(s)
end.
