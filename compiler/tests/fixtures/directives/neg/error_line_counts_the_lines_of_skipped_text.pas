program ErrorLineCountsSkippedText;
begin
{$IFDEF NOPE}
  { a comment
    over two lines }
  'a string'
  (* another
     comment *)
{$ELSE}
  Missing := 1;
{$ENDIF}
end.
