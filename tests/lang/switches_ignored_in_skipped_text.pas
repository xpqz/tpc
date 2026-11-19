program switches_ignored_in_skipped_text;
{$IFDEF NOPE}
{$Q+}{$G+}{$M}
{$ELSE}
{$S-,V-}
{$ENDIF}
begin
  writeln('skipped switches are not checked');
end.
