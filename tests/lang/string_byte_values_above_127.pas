program StringHighBytes;
var s: String;
begin
  s := #128#255#200;
  WriteLn(Length(s), ' ', Ord(s[1]), ' ', Ord(s[2]), ' ', Ord(s[3]));
  WriteLn(s > 'a', ' ', s[1] > 'z', ' ', s < #255#255)
end.
