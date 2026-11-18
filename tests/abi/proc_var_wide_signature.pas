program ProcVarWideSignature;
{$F+}
{ a procedural type with seven parameters of every kind, value and var interleaved: arguments
  in order, var arguments by address, values narrowed to their parameter's width }
type TWide = function(a: Byte; var b: Integer; c: Word; var d: LongInt; e: Boolean;
                      f: Char; var g: ShortInt): Integer;
var w: TWide;
function Wide(a: Byte; var b: Integer; c: Word; var d: LongInt; e: Boolean;
              f: Char; var g: ShortInt): Integer;
begin
  b := b + a;
  d := d + c;
  if e then g := g + 1 else g := g - 1;
  Wide := Ord(f) + b
end;
var b: Integer; d: LongInt; g: ShortInt; i: Integer;
begin
  w := Wide;
  b := 1; d := 70000; g := 127;
  WriteLn(w(255, b, 65535, d, True, 'A', g), ' ', b, ' ', d, ' ', g);
  i := 300;
  WriteLn(w(i, b, i, d, False, Chr(66), g), ' ', b, ' ', d, ' ', g)
end.
