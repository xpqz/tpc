program SkippedTextIsScanned;
{ in skipped text only strings and comments are recognised; the rest is not even checked }
begin
{$IFDEF NOPE}
  'unterminated string runs to the end of the line
  ? ! ~ 1e 99999999999 #300 $ ^ ` \ @@ 'x
  'a string hides {$ENDIF} and {$ELSE}' (* a comment hides {$ENDIF} *) { and so does {$ELSE} }
  (* braces { inside a star comment *) } stray
{$ELSE}
  WriteLn('else part');
{$ENDIF}
  WriteLn('done');
end.
