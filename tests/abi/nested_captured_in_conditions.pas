program NestedCapturedInConditions;
{ captured variables as operands of while, repeat, case and the short-circuit operators in the
  nested routine; a captured Boolean and a captured Char }
procedure Outer(limit: Integer);
var n: Integer; done: Boolean; c: Char;
  procedure Run;
  begin
    while n < limit do n := n + 3;
    repeat n := n - 2 until n < 10;
    case n of
      0..4: c := 'L';
      5..9: c := 'M'
    else c := 'H'
    end;
    done := (n > 3) and (c <> 'H') or (limit = 0)
  end;
begin
  n := 0; done := False; c := '?';
  Run;
  WriteLn(n, ' ', c, ' ', done)
end;
begin
  Outer(20);
  Outer(4);
  Outer(0)
end.
