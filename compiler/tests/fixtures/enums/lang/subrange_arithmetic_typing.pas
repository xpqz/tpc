program SubrangeArithmeticTyping;
type Hi = 200..250; Lo = -5..5; Wd = 0..60000; Lg = -1..40000;
var h: Hi; l: Lo; w: Wd; g: Lg;
begin
  h := 250; l := -4; w := 60000; g := 40000;
  WriteLn(h + h, ' ', h * h, ' ', -h, ' ', h - 251);
  WriteLn(l * l, ' ', -l, ' ', l - 32767, ' ', Abs(l));
  WriteLn(w + w, ' ', w * 2, ' ', -w, ' ', w + l);
  WriteLn(g + g, ' ', g * g, ' ', -g, ' ', g div 3, ' ', g mod 7);
  WriteLn(h div 7, ' ', h mod 7, ' ', l div 2, ' ', l mod 3)
end.
