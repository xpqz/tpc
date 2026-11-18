program CharLiteralsAndCodes;
var c: Char;
begin
  c := #65;
  WriteLn(c);
  WriteLn(#72#105);
  WriteLn('a'#66'c');
  c := #$7A;
  WriteLn(c)
end.
