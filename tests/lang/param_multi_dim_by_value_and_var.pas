program ParamMatrix;
type MT = array[1..2, 1..2] of Integer;
function Det(m: MT): Integer; begin Det := m[1, 1] * m[2, 2] - m[1, 2] * m[2, 1] end;
procedure Transpose(var m: MT);
var t: Integer;
begin t := m[1, 2]; m[1, 2] := m[2, 1]; m[2, 1] := t end;
var m: MT;
begin
  m[1, 1] := 1; m[1, 2] := 2; m[2, 1] := 3; m[2, 2] := 4;
  WriteLn(Det(m));
  Transpose(m);
  WriteLn(m[1, 1], m[1, 2], m[2, 1], m[2, 2], ' ', Det(m))
end.
