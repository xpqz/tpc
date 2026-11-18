program ParamSmallArrays;
type B = array[1..3] of Boolean; CT = array['a'..'c'] of Char;
procedure Flip(b: B; var out: B);
var i: Integer;
begin
  for i := 1 to 3 do begin b[i] := not b[i]; out[i] := b[i] end
end;
var x, y: B; c: CT; i: Integer;
begin
  x[1] := True; x[2] := False; x[3] := True;
  Flip(x, y);
  for i := 1 to 3 do Write(x[i], ' ', y[i], ' ');
  WriteLn;
  c['a'] := 'x'; c['b'] := 'y'; c['c'] := 'z';
  WriteLn(c['c'], c['b'], c['a'])
end.
