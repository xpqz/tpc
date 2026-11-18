program ProcValueParams;
procedure Show(a: Integer; b: LongInt; c: Char; d: Boolean);
begin
  WriteLn(a, ' ', b, ' ', c, ' ', d)
end;
procedure Many(a, b, c, d, e, f: Integer);
begin
  WriteLn(a, b, c, d, e, f);
  WriteLn(a + b * c - d * e + f)
end;
begin
  Show(1, 2, 'x', True);
  Show(-1, 100000, 'y', False);
  Many(1, 2, 3, 4, 5, 6);
  Many(6, 5, 4, 3, 2, 1)
end.
