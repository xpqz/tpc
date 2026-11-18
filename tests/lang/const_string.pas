program ConstString;
const
  Msg = 'hello';
  Quote = 'it''s';
begin
  WriteLn(Msg);
  WriteLn(Quote);
  WriteLn(Msg, ', ', Quote)
end.
