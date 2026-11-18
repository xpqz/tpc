program StringArray;
var names: array[1..4] of String[6]; w: array[0..2] of String; i: Integer; s: String;
begin
  names[1] := 'delta'; names[2] := 'alpha'; names[3] := 'charlie'; names[4] := 'bravo';
  for i := 1 to 4 do Write(names[i], '(', Length(names[i]), ') ');
  WriteLn;
  w[0] := 'x'; w[1] := w[0] + 'y'; w[2] := w[1] + w[0];
  WriteLn(w[2], ' ', w[2][2], ' ', names[3][6], names[3][1]);
  s := names[1]; names[1] := names[2]; names[2] := s;
  WriteLn(names[1], names[2], ' ', SizeOf(names), ' ', SizeOf(w))
end.
