var i: Integer; c: Word;
procedure P(var c: Word); begin Val('1x', i, c); writeln(c); end;
begin P(c); Val('12', i, c); writeln(i, c); end.
