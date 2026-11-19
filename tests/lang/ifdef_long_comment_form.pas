program IfdefLongCommentForm;
(*$DEFINE STAR*)
begin
(*$IFDEF STAR*)
  WriteLn('star form');
(*$ELSE*)
  WriteLn('wrong');
(*$ENDIF*)
end.
