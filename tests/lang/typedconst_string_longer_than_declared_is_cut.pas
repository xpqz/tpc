program TypedconstStringLongerThanDeclaredIsCut;
const
  s: string[3] = 'toolong';
begin
  WriteLn(s, ' ', Length(s));
end.
