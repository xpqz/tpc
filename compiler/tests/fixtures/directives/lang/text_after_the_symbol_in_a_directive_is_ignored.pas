program TextAfterTheSymbol;
{$DEFINE X trailing words}
begin
{$IFDEF X Y}
  WriteLn('X is the symbol');
{$ENDIF junk}
{$IFNDEF Y}
  WriteLn('Y is not defined');
{$ELSE the else may say anything}
  WriteLn('wrong');
{$ENDIF}
end.
