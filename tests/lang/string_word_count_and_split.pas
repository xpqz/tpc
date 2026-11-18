program WordCount;
var s, w: String; n, p: Integer;
begin
  s := '  the quick  brown fox   jumps ';
  n := 0;
  while Length(s) > 0 do begin
    while (Length(s) > 0) and (s[1] = ' ') do Delete(s, 1, 1);
    if Length(s) > 0 then begin
      p := Pos(' ', s);
      if p = 0 then p := Length(s) + 1;
      w := Copy(s, 1, p - 1);
      Delete(s, 1, p - 1);
      n := n + 1;
      WriteLn(n, ': ', w, ' ', Length(w))
    end
  end;
  WriteLn(n)
end.
