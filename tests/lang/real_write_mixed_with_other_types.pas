program RealWriteMix;
var x: Real; i: Integer; s: String; c: Char; b: Boolean;
begin
  x := 1.5;
  i := 3;
  s := 'ab';
  c := 'z';
  b := True;
  WriteLn(i, ' ', x:0:1, ' ', s, ' ', c, ' ', b, ' ', x:5:1, i:3);
  WriteLn(x:4:1, x:4:1, x:4:1);
  WriteLn(x:0:0, s, x:0:2, c);
  Write(x:3:1);
  Write(i);
  WriteLn
end.
