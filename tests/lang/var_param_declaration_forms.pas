program VarParamDeclarationForms;
{ var applies to every name up to the next semicolon; value and var groups interleave;
  a forward declaration may be completed without repeating the parameter list }
procedure Mix(a: Integer; var b, c: Byte; d: LongInt; var e: Boolean);
forward;
procedure Mix;
begin
  b := b + a;
  c := c + b;
  d := d + c;
  e := d > 100;
  WriteLn(a, ' ', b, ' ', c, ' ', d, ' ', e)
end;
var x, y: Byte; f: Boolean;
begin
  x := 1; y := 2;
  Mix(10, x, y, 50, f);
  WriteLn(x, ' ', y, ' ', f);
  Mix(200, x, y, 50, f);
  WriteLn(x, ' ', y, ' ', f)
end.
