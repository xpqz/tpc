program StringLiteralsAndEscapes;
const Quote = '''';
var s: String;
begin
  s := 'it''s';
  WriteLn(s, ' ', Length(s));
  WriteLn(#65#66, ' ', 'a'#67'd', ' ', #72'i', ' ', Length(#0), Length(''''), Length(''));
  WriteLn(Quote, Quote + Quote, Length(Quote + 'x' + Quote));
  s := '''''';
  WriteLn(s, Length(s));
  s := #48#49#50;
  WriteLn(s, Length(s), s[2]);
  WriteLn('^A' , Length('^A'), Length(^A), Ord(^A));
  s := 'tab' + #9 + 'end';
  WriteLn(Length(s), Ord(s[4]))
end.
