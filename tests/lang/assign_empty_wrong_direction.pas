program assign_empty_wrong_direction;
var
  f: Text;
  s: string;
begin
  Assign(f, '');
  Reset(f);
  {$I-}
  WriteLn(f, 'x');
  {$I+}
  WriteLn(IOResult);
  Rewrite(f);
  {$I-}
  ReadLn(f, s);
  {$I+}
  WriteLn(IOResult);
end.
