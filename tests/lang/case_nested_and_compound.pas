program CaseNestedAndCompound;
var a, b: Integer;
begin
  for a := 1 to 2 do
    for b := 1 to 3 do
      case a of
        1: case b of
             1: Write('a1b1 ');
             2, 3: Write('a1b23 ')
           end;
        2: begin
             Write('a2 ');
             if b = 3 then WriteLn
           end
      end;
  WriteLn
end.
