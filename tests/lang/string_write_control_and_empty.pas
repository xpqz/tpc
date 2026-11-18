program StringWriteControl;
var s: String;
begin
  s := 'a' + #9 + 'b';
  WriteLn(s, Length(s));
  Write(''); Write('', ''); WriteLn('');
  WriteLn(#65#66, 'x' + #67);
  s := '''quoted''';
  WriteLn(s, Length(s))
end.
