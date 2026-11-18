program StringCompareHigh;
var a, b: String;
begin
  a := #200; b := 'a';
  WriteLn(a > b, ' ', a < b, ' ', a = b);
  a := #0; b := '';
  WriteLn(a > b, ' ', a < b, ' ', a = b, ' ', Length(a), Length(b));
  a := 'a' + #0; b := 'a';
  WriteLn(a > b, ' ', a = b);
  a := #0'b'; b := #0'a';
  WriteLn(a > b, ' ', a < b);
  a := #255; b := #254#255;
  WriteLn(a > b, ' ', a < b)
end.
