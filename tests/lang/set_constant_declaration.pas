program SetConst;
type CS = set of Char;
const
  Vowels = ['a', 'e', 'i', 'o', 'u'];
  Digits = ['0'..'9'];
  Small = [1..5];
var s: String; i, v, d: Integer; chars: CS; n: Integer;
begin
  s := 'abc123 education 42';
  v := 0; d := 0;
  for i := 1 to Length(s) do begin
    if s[i] in Vowels then v := v + 1;
    if s[i] in Digits then d := d + 1;
  end;
  WriteLn(v, ' ', d);
  chars := Vowels + Digits;
  WriteLn('a' in chars, ' ', '5' in chars, ' ', 'z' in chars);
  n := 3;
  WriteLn(n in Small, ' ', 7 in Small, ' ', Small = [1..5])
end.
