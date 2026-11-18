program StringPos;
var s: String;
begin
  s := 'abracadabra';
  WriteLn(Pos('a', s), ' ', Pos('bra', s), ' ', Pos('cad', s), ' ', Pos('abra', s), ' ', Pos('z', s));
  WriteLn(Pos('', s), ' ', Pos('abracadabra', s), ' ', Pos('abracadabrax', s), ' ', Pos('ra', s));
  WriteLn(Pos('a', ''), ' ', Pos('', ''), ' ', Pos('a', 'a'));
  WriteLn(Pos(s, s), ' ', Pos('dab', 'abracadabra'), ' ', Pos('A', s));
  WriteLn(Pos(Copy(s, 4, 3), s))
end.
