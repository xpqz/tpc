program Palindrome;
function Clean(s: String): String;
var r: String; i: Integer;
begin
  r := '';
  for i := 1 to Length(s) do
    if (s[i] >= 'a') and (s[i] <= 'z') then r := r + s[i]
    else if (s[i] >= 'A') and (s[i] <= 'Z') then r := r + Chr(Ord(s[i]) + 32);
  Clean := r
end;
function IsPal(s: String): Boolean;
var i: Integer; ok: Boolean;
begin
  ok := True;
  for i := 1 to Length(s) div 2 do
    if s[i] <> s[Length(s) + 1 - i] then ok := False;
  IsPal := ok
end;
begin
  WriteLn(IsPal(Clean('A man, a plan, a canal: Panama')));
  WriteLn(IsPal(Clean('Hello')));
  WriteLn(IsPal(''), IsPal('x'), IsPal('xx'), IsPal('xy'))
end.
